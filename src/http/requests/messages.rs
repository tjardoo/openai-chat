use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct StoreMessageRequest {
    #[validate(length(min = 5, max = 50))]
    pub content: String,
}
