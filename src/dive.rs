use openai_dive::v1::{
    api::Client,
    resources::chat::{ChatCompletionChoice, ChatCompletionParameters, ChatMessage},
};
use sqlx::{MySql, Pool};
use std::{env, error::Error};

use crate::models::{
    message::{Message, Role},
    model::Model,
};

pub async fn send_message(pool: &Pool<MySql>, chat_id: u32) -> Result<(), Box<dyn Error>> {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let model = get_model_by_id(&pool, chat_id).await?;

    let messages = get_messages_by_chat_id(&pool, chat_id).await?;

    let parameters = ChatCompletionParameters {
        model: model.name,
        messages: messages.into_iter().map(ChatMessage::from).collect(),
        max_tokens: Some(12),
        ..Default::default()
    };

    let client = Client::new(api_key);

    let result = client.chat().create(parameters).await.unwrap();

    add_completed_messages_to_chat(&pool, chat_id, result.choices).await?;

    Ok(())
}

async fn get_model_by_id(pool: &Pool<MySql>, id: u32) -> Result<Model, Box<dyn Error>> {
    let model = sqlx::query_as!(
        Model,
        "SELECT
            id,
            name,
            owned_by,
            created_at
        FROM
            models
        WHERE
            id = ?",
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(model)
}

async fn get_messages_by_chat_id(
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

async fn add_completed_messages_to_chat(
    pool: &Pool<MySql>,
    chat_id: u32,
    messages: Vec<ChatCompletionChoice>,
) -> Result<(), Box<dyn Error>> {
    for message in messages {
        add_message_to_chat(pool, chat_id, message).await?;
    }

    Ok(())
}

async fn add_message_to_chat(
    pool: &Pool<MySql>,
    chat_id: u32,
    message: ChatCompletionChoice,
) -> Result<(), Box<dyn Error>> {
    sqlx::query_as!(
        Message,
        "INSERT INTO messages (chat_id, role, content) VALUES (?, ?, ?)",
        chat_id,
        "assistant".to_string(),
        message.message.content
    )
    .execute(pool)
    .await?;

    Ok(())
}
