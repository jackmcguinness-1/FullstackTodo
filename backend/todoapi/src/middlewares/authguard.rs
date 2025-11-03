use rocket::request::{self, Request, FromRequest};
use rocket::http::Status;
use rocket::outcome::Outcome;
use sqlx::PgPool;

pub struct AuthGuard {
    pub user_id: i32,
    pub token: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let cookies = req.cookies();
        let pool = req.rocket().state::<PgPool>().unwrap();
        
        // Get session_id from cookie
        let token = match cookies.get("auth") {
            Some(cookie) => {
                println!("cookie found: {}", cookie.value().to_string());
                cookie.value().to_string()
            },
            None => {
                println!("no cookie found!");
                return Outcome::Error((Status::Unauthorized, ()))
            },
        };
        
        // Look up session in database
        let session = sqlx::query!(
            r#"
            SELECT user_id, expires_at
            FROM sessions
            WHERE token = $1 AND expires_at > NOW()
            "#,
            token
        )
        .fetch_optional(pool)
        .await;

        println!("{:?}", session);
        
        match session {
            Ok(Some(session)) => {
                // Update last activity (optional)
                let _ = sqlx::query!(
                    "UPDATE sessions SET last_activity_at = NOW() WHERE token = $1",
                    token
                )
                .execute(pool)
                .await;
                
                Outcome::Success(AuthGuard {
                    user_id: session.user_id,
                    token,
                })
            }
            _ => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}