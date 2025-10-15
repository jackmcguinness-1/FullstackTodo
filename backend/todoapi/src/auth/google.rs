use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use rocket::serde::{json::Json, Deserialize};
use base64::{Engine, engine::general_purpose::STANDARD_NO_PAD};
use std::{collections::HashSet, error::Error};

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
    pub email: Option<String>,
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
#[post("/auth/google", data = "<body>")]
pub async fn google_auth(body: Json<JwtCredential>) -> String {
    println!("{:?}", body);
    let id_token = &body.credential;

    match verify_google_token(id_token, GOOGLE_CLIENT_ID).await {
        Ok(claims) => format!("Verified"),
        Err(e) => format!("Not Verified {}", e)
    }
}