//TODO: this endpoint should be disabled in prod, its just to quickly create a user and get a token for development purposes

use rocket::{http::CookieJar, post, serde::json::Json};
use serde::{Deserialize, Serialize};
use crate::{models::user::UserCreation, routes::auth::{providers::AuthProvider, util}};
use sqlx::PgPool;
use rocket::{
    State,
    http::Status,
};
use super::util::create_token;

#[derive(Deserialize)]
pub struct LoginDevReq {
    pub username: String,
    pub email: String
}

#[derive(Serialize)]
pub struct LoginDevRes {
    pub token: String,
}

#[post("/dev", data="<body>")]
pub async fn login_dev_endpoint(pool: &State<PgPool>, body: Json<LoginDevReq>, jar: &CookieJar<'_>) -> Result<Json<LoginDevRes>, Status> {
    let user_creation = UserCreation{
        email: body.email.clone(),
        username: body.username.clone(),
        auth_provider_id: AuthProvider::Dev.into(),
        auth_provider_user_id: None,
    };
    
    let user = util::create_user(pool, user_creation)
        .await
        .map_err(|err| {
            println!("{}", err);
            Status::BadRequest
        })?;

    let token = create_token(pool, jar, &user)
        .await
        .map_err(|err| {
            println!("{}", err);
            Status::BadRequest
        })?;

    Ok(Json::from(LoginDevRes{token}))
}