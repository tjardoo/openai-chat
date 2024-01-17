use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_streams::StreamBodyAs;
use std::sync::Arc;

use crate::{
    dive::{get_stream, source_openai_stream},
    http::{requests::messages::StoreMessageRequest, validation::ValidatedJson},
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

pub async fn store(
    Path(chat_id): Path<u32>,
    State(state): State<Arc<AppState>>,
    ValidatedJson(request): ValidatedJson<StoreMessageRequest>,
) -> impl IntoResponse {
    sqlx::query_as!(
        Message,
        "INSERT INTO messages (chat_id, role, content) VALUES (?, ?, ?)",
        chat_id,
        "user",
        request.content,
    )
    .execute(&state.pool)
    .await
    .unwrap()
    .last_insert_id();

    let stream = get_stream(&state.pool, chat_id, &request.model).await;

    StreamBodyAs::text(source_openai_stream(stream))
}
