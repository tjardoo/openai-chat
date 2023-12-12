use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    models::message::{Message, Role},
    state::{AppState, JsonError},
};

pub async fn index(
    Path(id): Path<u32>,
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<Vec<Message>>), (StatusCode, Json<JsonError>)> {
    match sqlx::query_as!(
        Message,
        "SELECT
            id,
            chat_id,
            role AS \"role: Role\",
            content,
            created_at
        FROM
            messages
        WHERE
            chat_id = ?",
        id
    )
    .fetch_all(&state.pool)
    .await
    {
        Ok(messages) => Ok((StatusCode::OK, Json(messages))),
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
    Path((chat_id, message_id)): Path<(u32, u32)>,
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<Message>), (StatusCode, Json<JsonError>)> {
    match sqlx::query_as!(
        Message,
        "SELECT
            id,
            chat_id,
            role AS \"role: Role\",
            content,
            created_at
        FROM
            messages
        WHERE
            chat_id = ? AND
            id = ?",
        chat_id,
        message_id
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
