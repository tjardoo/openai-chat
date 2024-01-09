use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;

use crate::{
    http::{requests::messages::StoreAssistantMessageRequest, validation::ValidatedJson},
    models::message::{Message, Role},
    state::{AppState, JsonError},
};

pub async fn store(
    Path(chat_id): Path<u32>,
    State(state): State<Arc<AppState>>,
    ValidatedJson(request): ValidatedJson<StoreAssistantMessageRequest>,
) -> Result<(StatusCode, Json<Message>), (StatusCode, Json<JsonError>)> {
    let last_inserted_id = sqlx::query_as!(
        Message,
        "INSERT INTO messages (chat_id, role, content) VALUES (?, ?, ?)",
        chat_id,
        "assistant",
        request.content,
    )
    .execute(&state.pool)
    .await
    .unwrap()
    .last_insert_id();

    let message = sqlx::query_as!(
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
            id = ?",
        last_inserted_id
    )
    .fetch_one(&state.pool)
    .await
    .unwrap();

    Ok((StatusCode::CREATED, Json(message)))
}
