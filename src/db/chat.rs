use openai_dive::v1::{
    api::Client,
    resources::chat::{ChatCompletionParameters, ChatMessage},
};
use sqlx::{MySql, MySqlPool, Pool};
use std::{env, error::Error};

use crate::models::message::{Message, Role};

pub async fn get_messages_by_chat_id(
    pool: &Pool<MySql>,
    chat_id: u32,
) -> Result<Vec<Message>, Box<dyn Error>> {
    let messages = sqlx::query_as!(
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
    .await?;

    Ok(messages)
}

pub async fn add_message_to_chat(
    pool: &Pool<MySql>,
    chat_id: u32,
    message: &str,
) -> Result<(), Box<dyn Error>> {
    sqlx::query_as!(
        Message,
        "INSERT INTO messages (chat_id, role, content) VALUES (?, ?, ?)",
        chat_id,
        "assistant".to_string(),
        message,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_chat_last_used_model(
    pool: &sqlx::MySqlPool,
    chat_id: u32,
    model: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE chats SET last_used_model = ? WHERE id = ?",
        model,
        chat_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_chat_title(pool: &MySqlPool, chat_id: u32, model: &str) {
    let openai_chat_summary_enabled =
        env::var("OPENAI_CHAT_SUMMARY_ENABLED").expect("$OPENAI_CHAT_SUMMARY_ENABLED is not set");

    if openai_chat_summary_enabled != "true" {
        return;
    }

    let messages = get_messages_by_chat_id(pool, chat_id).await.unwrap();

    if messages.len() != 1 {
        return;
    }

    let mut messages: Vec<ChatMessage> = messages.into_iter().map(ChatMessage::from).collect();
    messages.push(ChatMessage {
        content: Some("Summarize/Explain the first question. Limit to 40 characters.".to_string()),
        ..Default::default()
    });

    let parameters = ChatCompletionParameters {
        model: model.to_string(),
        messages,
        ..Default::default()
    };

    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let chat_completion_response = client.chat().create(parameters).await.unwrap();

    let new_title = chat_completion_response
        .choices
        .iter()
        .filter_map(|choice| choice.message.content.clone())
        .collect::<Vec<String>>()
        .join("");

    sqlx::query!(
        "UPDATE chats SET title = ? WHERE id = ?",
        new_title,
        chat_id
    )
    .execute(pool)
    .await
    .unwrap();
}
