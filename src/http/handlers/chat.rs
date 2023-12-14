use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use sqlx::{MySql, Pool};

use crate::{
    models::{chat::Chat, message::Message, message::Role},
    state::AppState,
};

pub async fn show(Path(id): Path<u32>, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let template = ChatTemplate {
        chat: get_chat(&state.pool, id).await,
        messages: get_messages_by_chat_id(&state.pool, id).await,
    };

    Html(template.render().unwrap())
}

pub async fn get_chat(pool: &Pool<MySql>, chat_id: u32) -> Chat {
    let chat = sqlx::query_as!(
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
        chat_id
    )
    .fetch_one(pool)
    .await
    .unwrap();

    chat
}

pub async fn get_messages_by_chat_id(pool: &Pool<MySql>, chat_id: u32) -> Vec<Message> {
    let messsages = sqlx::query_as!(
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
        chat_id
    )
    .fetch_all(pool)
    .await
    .unwrap();

    messsages
}

#[derive(Template)]
#[template(path = "chat.html")]
struct ChatTemplate {
    chat: Chat,
    messages: Vec<Message>,
}
