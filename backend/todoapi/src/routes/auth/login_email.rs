use rocket::post;

#[post("/login", data="<body>")]
pub async fn login_email_endpoint(body: String) -> String {
    "Email login is not yet implemented".to_string()
}