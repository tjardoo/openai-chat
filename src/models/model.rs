use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Model {
    pub id: u32,
    pub name: String,
    pub owned_by: String,
    pub created_at: DateTime<Utc>,
}
