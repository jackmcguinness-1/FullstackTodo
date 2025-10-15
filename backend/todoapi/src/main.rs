mod auth;

#[macro_use]
extern crate rocket;

use rocket::response::status::NotFound;
use rocket_cors::{CorsOptions, AllowedOrigins};

use crate::auth::google_auth;

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}


#[get("/hello/<name>/<age>")]
async fn hello(name: &str, age: u8) -> Result<String, NotFound<String>> {
    match name {
        "jack" => Ok(format!("Hello {} age {}", name, age)),
        _ => Err(NotFound("oh no".to_string()))
    }
}

#[launch]
fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:3000"]);
    let cors = CorsOptions {
        allowed_origins,
        allow_credentials: true,
        ..Default::default()
    }.to_cors().expect("CORS configuration failed");

    rocket::build()
        .mount("/", routes![index, hello, google_auth])
        .attach(cors)
}