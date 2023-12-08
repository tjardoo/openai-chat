use crate::AppState;
use askama::Template;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Form,
};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, sync::Arc};

use super::templating::HtmlTemplate;

pub async fn home() -> impl IntoResponse {
    let template = HomeTemplate {};

    HtmlTemplate(template)
}

pub async fn help() -> impl IntoResponse {
    let template = HelpTemplate {};

    HtmlTemplate(template)
}

pub async fn add_todo(
    State(state): State<Arc<AppState>>,
    Form(request): Form<TodoRequest>,
) -> impl IntoResponse {
    let mut lock = state.todos.lock().await;

    lock.push(request.todo);

    let template = TodoListTemplate {
        todos: lock
            .iter()
            .enumerate()
            .map(|(id, description)| Todo {
                id: id as u32,
                description: description.clone(),
            })
            .collect::<Vec<Todo>>(),
    };

    HtmlTemplate(template)
}

pub async fn get_todo(
    Path(id): Path<u32>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let lock = state.todos.lock().await;

    if let Some(todo) = lock.get(id as usize).cloned() {
        todo
    } else {
        "Todo not found".to_string()
    }
}

#[derive(Serialize, Deserialize)]
pub struct TodoRequest {
    todo: String,
}

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub description: String,
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

#[derive(Template)]
#[template(path = "help.html")]
struct HelpTemplate;

#[derive(Template)]
#[template(path = "todo-list.html")]
pub struct TodoListTemplate {
    pub todos: Vec<Todo>,
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.id, self.description)
    }
}
