use crate::AppState;
use askama::Template;
use axum::{extract::State, response::IntoResponse, Form};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::templating::HtmlTemplate;

pub async fn home() -> impl IntoResponse {
    let template = HomeTemplate {};

    HtmlTemplate(template)
}

pub async fn help() -> impl IntoResponse {
    let template = HelpTemplate {};

    HtmlTemplate(template)
}

pub async fn api() -> &'static str {
    "Hello"
}

pub async fn add_todo(
    State(state): State<Arc<AppState>>,
    Form(request): Form<TodoRequest>,
) -> impl IntoResponse {
    let mut lock = state.todos.lock().await;

    lock.push(request.todo);

    let template = TodoListTemplate {
        todos: lock.clone(),
    };

    HtmlTemplate(template)
}

#[derive(Serialize, Deserialize)]
pub struct TodoRequest {
    todo: String,
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
    pub todos: Vec<String>,
}
