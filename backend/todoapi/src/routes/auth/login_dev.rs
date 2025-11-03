//TODO: this endpoint should be disabled in prod, its just to quickly create a user and get a token for development purposes

use super::util::create_token;
use crate::{
    models::user::UserCreation,
    routes::auth::{providers::AuthProvider, util},
};
use rocket::{http::CookieJar, post, serde::json::Json};
use rocket::{http::Status, State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct LoginDevReq {
    pub username: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct LoginDevRes {
    pub token: String,
}

#[post("/dev", data = "<body>")]
pub async fn login_dev_endpoint(
    pool: &State<PgPool>,
    body: Json<LoginDevReq>,
    jar: &CookieJar<'_>,
) -> Result<Json<LoginDevRes>, Status> {
    let user_creation = UserCreation {
        email: body.email.clone(),
        username: body.username.clone(),
        auth_provider_id: AuthProvider::Dev.into(),
        auth_provider_user_id: None,
    };

    let mut user_result = util::create_user(pool, &user_creation)
        .await
        .map_err(|err| {
            println!("{}", err);
            Status::BadRequest
        });

    if let Err(e) = user_result {
        // the user may already exist in which case we should try to retrieve the user instead
        match util::get_user_from_email_address(pool, &user_creation.email, AuthProvider::Dev)
            .await
            .map_err(|err| {
                println!("{}", err);
                Status::BadRequest
            }) {
            Ok(Some(u)) => {
                user_result = Ok(u);
            }
            _ => {}
        }
    }

    let user = user_result?;

    let token = create_token(pool, jar, &user).await.map_err(|err| {
        println!("{}", err);
        Status::BadRequest
    })?;

    Ok(Json::from(LoginDevRes { token }))
}
