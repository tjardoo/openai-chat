use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdateChatRequest {
    #[validate(length(min = 1, max = 50))]
    pub title: String,
}
