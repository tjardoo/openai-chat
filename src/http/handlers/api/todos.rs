use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;

use crate::{
    http::{
        requests::todos::{StoreTodoRequest, UpdateTodoRequest},
        validation::ValidatedJson,
    },
    models::todo::Todo,
    state::{AppState, JsonError},
};

pub async fn index(
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<Vec<Todo>>), (StatusCode, Json<JsonError>)> {
    match sqlx::query_as!(
        Todo,
        "SELECT
            id,
            title,
            description
        FROM
            todos"
    )
    .fetch_all(&state.pool)
    .await
    {
        Ok(todos) => Ok((StatusCode::OK, Json(todos))),
        Err(error) => Err((
            StatusCode::NOT_FOUND,
            Json(JsonError {
                code: StatusCode::NOT_FOUND.as_u16(),
                error: error.to_string(),
            }),
        )),
    }
}

pub async fn store(
    State(state): State<Arc<AppState>>,
    ValidatedJson(request): ValidatedJson<StoreTodoRequest>,
) -> Result<(StatusCode, Json<Todo>), (StatusCode, Json<JsonError>)> {
    let last_inserted_id = sqlx::query_as!(
        Todo,
        "INSERT INTO todos (title, description) VALUES (?, ?)",
        request.title,
        request.description
    )
    .execute(&state.pool)
    .await
    .unwrap()
    .last_insert_id();

    match sqlx::query_as!(
        Todo,
        "SELECT
            id,
            title,
            description
        FROM
            todos
        WHERE
            id = ?",
        last_inserted_id
    )
    .fetch_one(&state.pool)
    .await
    {
        Ok(todo) => Ok((StatusCode::CREATED, Json(todo))),
        Err(error) => Err((
            StatusCode::NOT_FOUND,
            Json(JsonError {
                code: StatusCode::NOT_FOUND.as_u16(),
                error: error.to_string(),
            }),
        )),
    }
}

pub async fn show(
    Path(id): Path<u32>,
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<Todo>), (StatusCode, Json<JsonError>)> {
    match sqlx::query_as!(
        Todo,
        "SELECT
            id,
            title,
            description
        FROM
            todos
        WHERE
            id = ?",
        id
    )
    .fetch_one(&state.pool)
    .await
    {
        Ok(todo) => Ok((StatusCode::OK, Json(todo))),
        Err(error) => Err((
            StatusCode::NOT_FOUND,
            Json(JsonError {
                code: StatusCode::NOT_FOUND.as_u16(),
                error: error.to_string(),
            }),
        )),
    }
}

pub async fn update(
    Path(id): Path<u32>,
    State(state): State<Arc<AppState>>,
    ValidatedJson(request): ValidatedJson<UpdateTodoRequest>,
) -> Result<(StatusCode, Json<Todo>), (StatusCode, Json<JsonError>)> {
    sqlx::query_as!(
        Todo,
        "UPDATE todos SET title = ?, description = ? WHERE id = ?",
        request.title,
        request.description,
        id
    )
    .execute(&state.pool)
    .await
    .unwrap();

    match sqlx::query_as!(
        Todo,
        "SELECT
            id,
            title,
            description
        FROM
            todos
        WHERE
            id = ?",
        id
    )
    .fetch_one(&state.pool)
    .await
    {
        Ok(todo) => Ok((StatusCode::OK, Json(todo))),
        Err(error) => Err((
            StatusCode::NOT_FOUND,
            Json(JsonError {
                code: StatusCode::NOT_FOUND.as_u16(),
                error: error.to_string(),
            }),
        )),
    }
}

pub async fn destroy(
    Path(id): Path<u32>,
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<String>), (StatusCode, Json<JsonError>)> {
    match sqlx::query_as!(
        Todo,
        "SELECT
            id,
            title,
            description
        FROM
            todos
        WHERE
            id = ?",
        id
    )
    .fetch_one(&state.pool)
    .await
    {
        Ok(_) => {
            sqlx::query_as!(Todo, "DELETE FROM todos where id = ?", id)
                .execute(&state.pool)
                .await
                .unwrap();

            Ok((StatusCode::NO_CONTENT, Json("deleted".to_string())))
        }
        Err(error) => Err((
            StatusCode::NOT_FOUND,
            Json(JsonError {
                code: StatusCode::NOT_FOUND.as_u16(),
                error: error.to_string(),
            }),
        )),
    }
}
