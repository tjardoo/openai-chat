use chrono::{DateTime, Utc};
use openai_dive::v1::resources::chat::ChatMessage;
use openai_dive::v1::resources::chat::ChatMessageContent;
use openai_dive::v1::resources::chat::Role as DiveRole;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use std::fmt::Display;
use validator::Validate;

#[derive(Serialize, Deserialize, FromRow, Validate)]
pub struct Message {
    pub id: u32,
    #[serde(skip_serializing)]
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

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::User => write!(f, "User"),
            Role::Assistant => write!(f, "Assistant"),
        }
    }
}

impl From<Message> for ChatMessage {
    fn from(message: Message) -> Self {
        ChatMessage {
            role: message.role.into(),
            content: ChatMessageContent::Text(message.content),
            tool_calls: None,
            name: None,
            tool_call_id: None,
        }
    }
}

impl From<Role> for DiveRole {
    fn from(role: Role) -> Self {
        match role {
            Role::User => DiveRole::User,
            Role::Assistant => DiveRole::Assistant,
        }
    }
}
