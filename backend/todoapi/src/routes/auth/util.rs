use crate::{models::user::{User, UserCreation}, routes::auth::providers::AuthProvider};
use sqlx::PgPool;
use uuid::Uuid;
use rocket::http::{CookieJar, Cookie};
use chrono::{Duration, Utc};
use anyhow::{Error, Result};

pub async fn get_user_from_email_address(pool: &PgPool, email: &str, provider: AuthProvider) -> Result<Option<User>> {
    let auth_provider_id: i32 = provider.into();
    let user = sqlx::query_as!(
        User,
        r#"SELECT id, email, username, auth_provider_id, auth_provider_user_id FROM users WHERE email = $1 AND auth_provider_id = $2"#,
        email,
        auth_provider_id,
    )
    .fetch_optional(pool)
    .await?;

    return Ok(user);
}

pub async fn create_token(pool: &PgPool, jar: &CookieJar<'_>, user: &User) -> anyhow::Result<String> {
    let token = Uuid::new_v4();
    // first try to put the token into our DB
    sqlx::query!(
        "INSERT INTO sessions (token, user_id, expires_at) VALUES ($1, $2, $3);",
        token.to_string(),
        user.id,
        &(Utc::now() + Duration::days(7)),
    )
    .execute(pool)
    .await?;

    jar.add(Cookie::new("auth", token.to_string()));

    Ok(token.to_string())
}

pub async fn create_user(pool: &PgPool, user_creation: &UserCreation) -> Result<User> {
    let user = sqlx::query_as!(
        User,
        r#"INSERT INTO users (email, username, auth_provider_id, auth_provider_user_id)
        VALUES ($1, $2, $3, $4)
        RETURNING id, email, username, auth_provider_id, auth_provider_user_id"#,
        user_creation.email,
        user_creation.username,
        user_creation.auth_provider_id,
        user_creation.auth_provider_user_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}