use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Form, Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{models::Todo, state::AppState};

use super::templating::HtmlTemplate;

pub async fn home() -> impl IntoResponse {
    let template = HomeTemplate {};

    HtmlTemplate(template)
}

pub async fn help() -> impl IntoResponse {
    let template = HelpTemplate {};

    HtmlTemplate(template)
}

pub async fn index_todo(
    State(_state): State<Arc<AppState>>,
    Form(_request): Form<TodoRequest>,
) -> impl IntoResponse {
    let template = TodoListTemplate { todos: vec![] };

    HtmlTemplate(template)
}

pub async fn show_todo(
    Path(id): Path<u32>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, Todo>("SELECT * FROM todos where id = ?")
        .bind(id)
        .fetch_one(&state.pool)
        .await
    {
        Ok(todo) => Ok((StatusCode::OK, Json(todo))),
        Err(error) => Err((StatusCode::NOT_FOUND, error.to_string())),
    }
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
    pub todos: Vec<Todo>,
}
