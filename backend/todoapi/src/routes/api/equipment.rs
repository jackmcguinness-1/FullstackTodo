use rocket::{
    get,
    serde::json::Json,
    http::Status,
    State,
};
use sqlx::PgPool;

use crate::middlewares::authguard::AuthGuard;

#[get("/equipment")]
pub async fn get_equipment(guard: AuthGuard, pool: &State<PgPool>) -> Result<Json<Vec<String>>, Status> {
let names = sqlx::query_scalar!(
        r"SELECT name 
        FROM equipments JOIN owned_equipments on equipments.id = owned_equipments.equipment_id
        WHERE owned_equipments.user_id = $1;",
        guard.user_id,
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|err| {
        println!("{:?}", err);
        Status::BadRequest
    })?;

    return Ok(Json::from(names));
}