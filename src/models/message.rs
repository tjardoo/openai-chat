use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: u32,
    pub chat_id: u32,
    pub role: Role,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "role", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
}
