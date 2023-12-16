use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    models::chat::Chat,
    state::{AppState, JsonError},
};

pub async fn index(
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<Vec<Chat>>), (StatusCode, Json<JsonError>)> {
    match sqlx::query_as!(
        Chat,
        "SELECT
            id,
            title,
            model_id,
            external_id,
            created_at
        FROM
            chats"
    )
    .fetch_all(&state.pool)
    .await
    {
        Ok(chats) => Ok((StatusCode::OK, Json(chats))),
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
) -> Result<(StatusCode, Json<Chat>), (StatusCode, Json<JsonError>)> {
    let last_inserted_id = sqlx::query_as!(Chat, "INSERT INTO chats (created_at) VALUES (NOW())")
        .execute(&state.pool)
        .await
        .unwrap()
        .last_insert_id();

    match sqlx::query_as!(
        Chat,
        "SELECT
            id,
            title,
            model_id,
            external_id,
            created_at
        FROM
            chats
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
) -> Result<(StatusCode, Json<Chat>), (StatusCode, Json<JsonError>)> {
    match sqlx::query_as!(
        Chat,
        "SELECT
            id,
            title,
            model_id,
            external_id,
            created_at
        FROM
            chats
        WHERE
            id = ?",
        id
    )
    .fetch_one(&state.pool)
    .await
    {
        Ok(chat) => Ok((StatusCode::OK, Json(chat))),
        Err(error) => Err((
            StatusCode::NOT_FOUND,
            Json(JsonError {
                code: StatusCode::NOT_FOUND.as_u16(),
                error: error.to_string(),
            }),
        )),
    }
}
