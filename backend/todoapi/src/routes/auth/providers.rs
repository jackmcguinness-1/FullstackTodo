pub enum AuthProvider {
    Dev,
    Email,
    Google,
}

impl Into<i32> for AuthProvider {
    fn into(self) -> i32 {
        match self {
            AuthProvider::Dev => 0,
            AuthProvider::Email => 1,
            AuthProvider::Google => 2
        }
    }
}