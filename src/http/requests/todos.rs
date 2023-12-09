use serde::Deserialize;

#[derive(Deserialize)]
pub struct StoreTodoRequest {
    pub description: String,
}

#[derive(Deserialize)]
pub struct UpdateTodoRequest {
    pub description: String,
}
