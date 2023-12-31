use futures::StreamExt;
use openai_dive::v1::{
    api::Client,
    resources::chat::{ChatCompletionParameters, ChatMessage},
};
use sqlx::{MySql, Pool};
use std::{env, error::Error};

use crate::models::message::{Message, Role};

pub async fn send_message(
    pool: &Pool<MySql>,
    chat_id: u32,
    model_name: &str,
) -> Result<String, Box<dyn Error>> {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let messages = get_messages_by_chat_id(&pool, chat_id).await?;

    let messages: Vec<ChatMessage> = messages.into_iter().map(ChatMessage::from).collect();

    let parameters = ChatCompletionParameters {
        model: model_name.to_string(),
        messages,
        ..Default::default()
    };

    let client = Client::new(api_key);

    let mut stream = client.chat().create_stream(parameters).await.unwrap();

    let mut message = String::new();

    while let Some(response) = stream.next().await {
        match response {
            Ok(chat_response) => chat_response.choices.iter().for_each(|choice| {
                if let Some(content) = &choice.delta.content {
                    message.push_str(content);

                    print!("{}", content);
                }
            }),
            Err(e) => eprintln!("{}", e),
        }
    }

    add_message_to_chat(&pool, chat_id, &message).await?;

    Ok(message)
}

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

async fn add_message_to_chat(
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
