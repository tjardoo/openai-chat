use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Chat {
    pub id: u32,
    pub open_ai_id: String,
    pub model_id: u32,
    pub created_at: DateTime<Utc>,
}
