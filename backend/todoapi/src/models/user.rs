#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name="auth_provider", rename_all="lowercase")]
pub enum AuthProvider {
    Google
}

// this is the User struct that we will query from the database
#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub provider: AuthProvider,
    pub provider_id: Option<String>,
}