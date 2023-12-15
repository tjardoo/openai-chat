use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
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
) -> Result<(StatusCode, Json<Message>), (StatusCode, Json<JsonError>)> {
    let last_inserted_id = sqlx::query_as!(
        Message,
        "INSERT INTO messages (chat_id, role, content) VALUES (?, ?, ?)",
        chat_id,
        "user".to_string(),
        request.content
    )
    .execute(&state.pool)
    .await
    .unwrap()
    .last_insert_id();

    // @todo add to db
    // let chat_completion_response = crate::dive::send_message(&state.pool, chat_id)
    //     .await
    //     .unwrap();

    // println!("{:?}", chat_completion_response);

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
