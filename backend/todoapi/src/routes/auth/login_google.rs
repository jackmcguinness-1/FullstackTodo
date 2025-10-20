use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use rocket::{http::{Cookie, CookieJar, Status}, post, serde::{json::Json, Deserialize}, State};
use std::{collections::HashSet, error::Error};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::user::{User, AuthProvider};
use chrono::Utc;
use anyhow::Result;

#[derive(Deserialize, Debug)]
pub struct Jwks {
    pub keys: Vec<Jwk>
}

#[derive(Deserialize, Debug)]
pub struct Jwk {
    pub e: String,
    pub n: String,
    pub kid: String,
}

#[derive(Deserialize, Debug)]
pub struct Claims {
    pub iss: String,
    pub aud: String,
    pub sub: String,
    pub email: String,
    pub exp: usize,
    pub iat: usize,
    pub name: Option<String>,
    pub picture: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JwtCredential {
    pub credential: String,
}

fn jwk_to_decoding_key(jwk: &Jwk) -> Result<DecodingKey, Box<dyn Error>> {
    let key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)?;
    Ok(key) 
}

async fn verify_google_token(id_token: &str, client_id: &str) -> Result<Claims, Box<dyn Error>> {
    let header = decode_header(id_token)?;
    let kid = header.kid.ok_or("missing kid")?;

    let jwks = fetch_google_jwks().await?;

    let jwk = jwks
        .keys
        .into_iter()
        .find(|k| k.kid == kid)
        .ok_or("No matching JWK found")?;

    let decoding_key = jwk_to_decoding_key(&jwk)?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[client_id]);

    let mut issuers = HashSet::new();
    issuers.insert("https://accounts.google.com".to_string());
    validation.iss = Some(issuers);

    let token_data = decode::<Claims>(id_token, &decoding_key, &validation)?;

    Ok(token_data.claims)
}

pub async fn fetch_google_jwks() -> Result<Jwks, reqwest::Error> {
    let body = reqwest::get("https://www.googleapis.com/oauth2/v3/certs")
        .await?
        .json::<Jwks>()
        .await?;

    println!("{:?}", body);

    return Ok(body);
}

const GOOGLE_CLIENT_ID: &'static str = "594337534921-3arlg5tpasb984jt980p1tgdbgeckg8o.apps.googleusercontent.com";

// this function takes the JWT that google issues to the client and verifies it against
// googles public keys
#[post("/google", data = "<body>")]
pub async fn login_google_endpoint(body: Json<JwtCredential>, pool: &State<PgPool>, jar: &CookieJar<'_>) -> Result<String, Status> {
    println!("hit google auth");
    println!("{:?}", body);
    let id_token = &body.credential;

    let claims = match verify_google_token(id_token, GOOGLE_CLIENT_ID).await {
        Ok(claims) => claims,
        Err(e) => {
            eprintln!("token auth failed");
            return Err(Status::Unauthorized);
        }
    };

    let user_result = find_user_from_email_address(pool, &claims.email, AuthProvider::Google).await;

    match user_result {
        Ok(Some(u)) => {
            // create a token 
            println!("creating token for existing user");
            return create_token(pool, jar, &u)
                .await
                .map_err(|_| Status::Unauthorized);
        },
        Ok(None) => {
            // create the account
            println!("creating new user");
            let new_user = create_user_from_google_claims(pool, &claims, AuthProvider::Google)
                .await
                .map_err(|_| Status::Unauthorized)?;
            // create a token for the user
            return create_token(pool, jar, &new_user)
                .await
                .map_err(|_| Status::Unauthorized);
        },
        Err(e) => {
            // log the error and return unauthorised
            eprintln!("{}", e);
            return Err(Status::Unauthorized)
        }
    };
}

async fn create_user_from_google_claims(pool: &PgPool, claims: &Claims, provider: AuthProvider) -> Result<User> {
    let user = sqlx::query_as!(
        User,
        r#"INSERT INTO users (email, username, provider, provider_id)
        VALUES ($1, $2, $3, $4)
        RETURNING id, email, username, provider as "provider: AuthProvider", provider_id"#,
        claims.email,
        claims.name,
        provider as AuthProvider,
        claims.sub

    )
    .fetch_one(pool)
    .await?;

    return Ok(user);
}

// this function can be put into a different package and reused for different auth providers
async fn find_user_from_email_address(pool: &PgPool, email: &str, provider: AuthProvider) -> Result<Option<User>> {
    let user = sqlx::query_as!(
        User,
        r#"SELECT id, email, username, provider as "provider: AuthProvider", provider_id FROM users WHERE email = $1 AND provider = $2"#,
        email,
        provider as AuthProvider,
    )
    .fetch_optional(pool)
    .await?;

    return Ok(user);
}

async fn create_token(pool: &PgPool, jar: &CookieJar<'_>, user: &User) -> anyhow::Result<String> {
    let token = Uuid::new_v4();
    // first try to put the token into our DB
    sqlx::query!(
        "INSERT INTO sessions (token, user_id, expires_at) VALUES ($1, $2, $3);",
        token.to_string(),
        user.id,
        &Utc::now(),
    )
    .execute(pool)
    .await?;

    jar.add(Cookie::new("auth", token.to_string()));

    Ok("tet".to_string())
}