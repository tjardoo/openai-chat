use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;

use crate::{
    http::{requests::chats::UpdateChatRequest, validation::ValidatedJson},
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
            last_used_model,
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
    let number_of_chats: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM chats")
        .fetch_one(&state.pool)
        .await
        .unwrap();

    let chat_name = format!("Chat {}", number_of_chats + 1);

    let last_inserted_id =
        sqlx::query_as!(Chat, "INSERT INTO chats (title) VALUES (?)", chat_name,)
            .execute(&state.pool)
            .await
            .unwrap()
            .last_insert_id();

    match sqlx::query_as!(
        Chat,
        "SELECT
            id,
            title,
            last_used_model,
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
        Ok(chat) => Ok((StatusCode::CREATED, Json(chat))),
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
            last_used_model,
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

pub async fn update(
    Path(id): Path<u32>,
    State(state): State<Arc<AppState>>,
    ValidatedJson(request): ValidatedJson<UpdateChatRequest>,
) -> Result<(StatusCode, Json<Chat>), (StatusCode, Json<JsonError>)> {
    sqlx::query_as!(
        Chat,
        "UPDATE chats SET title = ? WHERE id = ?",
        request.title,
        id
    )
    .execute(&state.pool)
    .await
    .unwrap();

    match sqlx::query_as!(
        Chat,
        "SELECT
            id,
            title,
            last_used_model,
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
        Ok(message) => Ok((StatusCode::CREATED, Json(message))),
        Err(error) => Err((
            StatusCode::NOT_FOUND,
            Json(JsonError {
                code: StatusCode::NOT_FOUND.as_u16(),
                error: error.to_string(),
            }),
        )),
    }
}

pub async fn delete(
    Path(id): Path<u32>,
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, String), (StatusCode, Json<JsonError>)> {
    sqlx::query_as!(Chat, "DELETE FROM chats WHERE id = ?", id)
        .execute(&state.pool)
        .await
        .unwrap();

    Ok((StatusCode::NO_CONTENT, "".to_string()))
}
