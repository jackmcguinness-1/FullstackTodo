use rocket::{
    get,
    post,
    serde::json::Json,
    http::Status,
    State,
};
use sqlx::{
    PgPool, Postgres, QueryBuilder
};

use crate::{middlewares::authguard::AuthGuard, models::Equipment};

#[get("/equipment")]
pub async fn get_equipment_endpoint(
    guard: AuthGuard, 
    pool: &State<PgPool>
) -> Result<Json<Vec<Equipment>>, Status> {
let names : Vec<Equipment> = sqlx::query_as!(
    Equipment,
    r"SELECT id, name FROM equipments;"
)
    .fetch_all(pool.inner())
    .await
    .map_err(|err| {
        println!("{:?}", err);
        Status::BadRequest
    })?;

    return Ok(Json::from(names));
}

#[post("/equipment", data="<body>")]
pub async fn post_equipment_endpoint(
    guard: AuthGuard, 
    pool: &State<PgPool>, 
    body: Json<Vec<String>>
) -> Result<(), Status> {
    let r = insert_into_equipments(body.into_inner(), pool.inner()).await;
    match r {
        Ok(()) => Ok(()),
        Err(()) => Err(Status::BadRequest)
    }
}

pub async fn insert_into_equipments(names: Vec<String>, pool: &PgPool) -> Result<(), ()> {
    let mut q = QueryBuilder::<Postgres>::new("INSERT INTO equipments (name) VALUES");
    q.push_values(names, |mut b, e: String| {
        b.push_bind(e);
    });

    match q.build().execute(pool).await {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("{}", e);
            Err(())
        }
    }
}