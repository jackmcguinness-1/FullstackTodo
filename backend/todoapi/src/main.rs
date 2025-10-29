mod routes;
mod models;
mod types;
mod middlewares;

use rocket::{launch, routes};
use rocket_cors::{CorsOptions, AllowedOrigins};
use crate::routes::auth::{
    login_google_endpoint,
    login_email_endpoint,
    register_email_endpoint,
    token_auth_endpoint,
};
use crate::routes::api::{
    equipment::get_equipment
};
use sqlx::PgPool;
use dotenv::dotenv;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:3000"]);
    let cors = CorsOptions {
        allowed_origins,
        allow_credentials: true,
        ..Default::default()
    }.to_cors().expect("CORS configuration failed");

    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Migrations failed");

    rocket::build()
        .manage(pool)
        .mount("/auth", routes![
            login_email_endpoint,
            register_email_endpoint,
            token_auth_endpoint,
        ])
        .mount("/auth/oauth", routes![
            login_google_endpoint,
        ])
        .mount("/api", routes![
            get_equipment,
        ])
        .attach(cors)
}