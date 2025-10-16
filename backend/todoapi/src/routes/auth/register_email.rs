use rocket::post;

#[post("/register", data="<body>")]
pub async fn register_email_endpoint(body: String) -> String {
    "Email registration is not yet implemented".to_string()
}