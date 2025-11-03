use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Equipment {
    pub id: i32,
    pub name: String,
}