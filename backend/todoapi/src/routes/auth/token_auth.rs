use rocket::{
    get,
    http::{Cookie, CookieJar, Status},
    State,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::middlewares::authguard::AuthGuard;

#[get("/me")]
pub async fn token_auth_endpoint(
    authguard: AuthGuard,
    jar: &CookieJar<'_>,
    pool: &State<PgPool>,
) -> Result<String, Status> {
    let auth_cookie = jar.get("auth");
    match auth_cookie {
        Some(cookie) => {
            ()
        },
        None => return Err(Status::Unauthorized)
    }
    // to check if we have a valid authentication token we need to check the cookies of the request
    Ok(String::from("auth success"))
}