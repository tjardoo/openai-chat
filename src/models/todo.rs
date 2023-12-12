use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub description: String,
}
