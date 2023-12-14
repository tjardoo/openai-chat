use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::message::Role;

#[derive(Deserialize, Serialize, Validate)]
pub struct StoreMessageRequest {
    pub role: Role,
    #[validate(length(min = 5, max = 50))]
    pub content: String,
}
