use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct StoreTodoRequest {
    #[validate(length(min = 5, max = 20))]
    pub title: String,
    #[validate(length(min = 10, max = 255))]
    pub description: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdateTodoRequest {
    #[validate(length(min = 5, max = 20))]
    pub title: String,
    #[validate(length(min = 10, max = 255))]
    pub description: String,
}
