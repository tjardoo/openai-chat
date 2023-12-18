use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

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
