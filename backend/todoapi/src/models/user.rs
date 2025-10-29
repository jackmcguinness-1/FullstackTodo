// this is the User struct that we will query from the database
#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub auth_provider_id: i32,
    pub auth_provider_user_id: Option<String>,
}

pub struct UserCreation {
    pub email: String,
    pub username: String,
    pub auth_provider_id: i32,
    pub auth_provider_user_id: Option<String>,
}