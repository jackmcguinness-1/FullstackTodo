#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name="auth_provider", rename_all="lowercase")]
pub enum AuthProvider {
    Google
}

impl Into<i32> for AuthProvider {
    fn into(self) -> i32 {
        match self {
            AuthProvider::Google => 0
        }
    }
}

// this is the User struct that we will query from the database
#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub auth_provider_id: i32,
    pub auth_provider_user_id: Option<String>,
}