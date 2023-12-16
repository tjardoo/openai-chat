use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct StoreMessageRequest {
    #[validate(length(min = 1, max = 2500))]
    pub content: String,
    pub model: String,
    pub max_tokens: Option<u32>,
}
