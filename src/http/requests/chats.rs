use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct StoreChatRequest {
    #[validate(length(min = 5, max = 50))]
    pub title: String,
    pub model_id: u32,
    pub external_id: String,
}
