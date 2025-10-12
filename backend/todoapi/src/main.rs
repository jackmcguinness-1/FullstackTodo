#[macro_use]
extern crate rocket;

use rocket::response::status::NotFound;

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> Result<String, NotFound<String>> {
    match name {
        "jack" => Ok(format!("Hello {} age {}", name, age)),
        _ => Err(NotFound("oh no".to_string()))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hello])
}