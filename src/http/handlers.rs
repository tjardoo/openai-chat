use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    Form, Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    models::Todo,
    state::{AppState, JsonError},
};

pub async fn home() -> impl IntoResponse {
    let template = HomeTemplate {};

    Html(template.render().unwrap())
}

pub async fn help() -> impl IntoResponse {
    let template = HelpTemplate {};

    Html(template.render().unwrap())
}

pub async fn store_todo(
    State(state): State<Arc<AppState>>,
    Form(request): Form<TodoRequest>,
) -> Result<(StatusCode, Json<Todo>), (StatusCode, String)> {
    let last_inserted_id = sqlx::query_as!(
        Todo,
        "INSERT INTO todos (description) VALUES (?)",
        request.description
    )
    .execute(&state.pool)
    .await
    .unwrap()
    .last_insert_id();

    match sqlx::query_as!(Todo, "SELECT * FROM todos where id = ?", last_inserted_id)
        .fetch_one(&state.pool)
        .await
    {
        Ok(todo) => Ok((StatusCode::CREATED, Json(todo))),
        Err(error) => Err((StatusCode::NOT_FOUND, error.to_string())),
    }
}

pub async fn index_todo(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match sqlx::query_as!(Todo, "SELECT * FROM todos")
        .fetch_all(&state.pool)
        .await
    {
        Ok(todos) => Ok((StatusCode::OK, Json(todos))),
        Err(error) => Err((StatusCode::NO_CONTENT, error.to_string())),
    }
}

pub async fn show_todo(
    Path(id): Path<u32>,
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<Todo>), (StatusCode, Json<JsonError>)> {
    match sqlx::query_as!(Todo, "SELECT * FROM todos where id = ?", id)
        .fetch_one(&state.pool)
        .await
    {
        Ok(todo) => Ok((StatusCode::OK, Json(todo))),
        Err(error) => Err((
            StatusCode::NOT_FOUND,
            Json(JsonError {
                error: error.to_string(),
            }),
        )),
    }
}

#[derive(Serialize, Deserialize)]
pub struct TodoRequest {
    description: String,
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
