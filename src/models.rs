use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use std::fmt::Display;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Todo {
    pub id: u32,
    pub description: String,
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.id, self.description)
    }
}
