use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

pub struct AppState {
    pub pool: MySqlPool,
}

#[derive(Deserialize, Serialize)]
pub struct JsonError {
    pub error: String,
}
