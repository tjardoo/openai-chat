use openai_dive::v1::{
    api::Client,
    resources::{
        chat::{ChatCompletionChoice, ChatCompletionParameters, ChatMessage},
        shared::Usage,
    },
};
use sqlx::{MySql, Pool};
use std::{env, error::Error};

use crate::models::message::{Message, Role};

pub async fn send_message(
    pool: &Pool<MySql>,
    chat_id: u32,
    model_name: String,
    max_tokens: Option<u32>,
) -> Result<Usage, Box<dyn Error>> {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let messages = get_messages_by_chat_id(&pool, chat_id).await?;

    let messages: Vec<ChatMessage> = messages.into_iter().map(ChatMessage::from).collect();

    let parameters = ChatCompletionParameters {
        model: model_name.clone(),
        messages,
        max_tokens,
        ..Default::default()
    };

    let client = Client::new(api_key);

    let result = client.chat().create(parameters).await.unwrap();

    add_completed_messages_to_chat(
        &pool,
        chat_id,
        result.choices,
        model_name,
        result.usage.completion_tokens,
    )
    .await?;

    Ok(result.usage)
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
            used_model,
            prompt_tokens AS \"prompt_tokens: u32\",
            completion_tokens AS \"completion_tokens: u32\",
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
    used_model: String,
    max_tokens: Option<u32>,
) -> Result<(), Box<dyn Error>> {
    for message in messages {
        add_message_to_chat(pool, chat_id, message, &used_model, &max_tokens).await?;
    }

    Ok(())
}

async fn add_message_to_chat(
    pool: &Pool<MySql>,
    chat_id: u32,
    message: ChatCompletionChoice,
    used_model: &str,
    completion_tokens: &Option<u32>,
) -> Result<(), Box<dyn Error>> {
    sqlx::query_as!(
        Message,
        "INSERT INTO messages (chat_id, role, content, used_model, completion_tokens) VALUES (?, ?, ?, ?, ?)",
        chat_id,
        "assistant".to_string(),
        message.message.content,
        used_model,
        completion_tokens
    )
    .execute(pool)
    .await?;

    Ok(())
}
