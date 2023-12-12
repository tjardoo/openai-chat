use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Chat {
    pub id: u32,
    pub title: String,
    pub model_id: u32,
    pub external_id: String,
    pub created_at: DateTime<Utc>,
}
