use rocket::get;

#[get("/me")]
pub async fn token_auth_endpoint() -> String {
    "Token authentication is not yet implemented".to_string()
}