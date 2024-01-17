use futures::{stream, Stream};
use openai_dive::v1::{
    api::Client,
    resources::chat::{ChatCompletionParameters, ChatMessage},
};
use openai_dive::v1::{error::APIError, resources::chat::ChatCompletionChunkResponse};
use sqlx::{MySql, MySqlPool, Pool};
use std::pin::Pin;
use std::{env, error::Error};
use tokio_stream::StreamExt;

use crate::db::chat::{
    add_message_to_chat, get_messages_by_chat_id, update_chat_last_used_model, update_chat_title,
};

type OpenAIStream =
    Pin<Box<dyn Stream<Item = Result<ChatCompletionChunkResponse, APIError>> + std::marker::Send>>;

pub fn source_openai_stream(stream: OpenAIStream) -> impl Stream<Item = String> {
    stream::unfold(stream, |mut s| async {
        match s.next().await {
            Some(Ok(chat_response)) => {
                let content = chat_response
                    .choices
                    .iter()
                    .filter_map(|choice| choice.delta.content.clone())
                    .collect::<Vec<String>>()
                    .join("");

                Some((content, s))
            }
            Some(Err(e)) => {
                eprintln!("{}", e);
                None
            }
            None => None,
        }
    })
}

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

pub async fn get_stream(
    pool: &MySqlPool,
    chat_id: u32,
    model_name: &str,
) -> Pin<Box<dyn Stream<Item = Result<ChatCompletionChunkResponse, APIError>> + Send>> {
    // @todo: move this logic
    update_chat_title(&pool, chat_id, model_name).await;

    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let messages = get_messages_by_chat_id(&pool, chat_id).await.unwrap();

    let messages: Vec<ChatMessage> = messages.into_iter().map(ChatMessage::from).collect();

    // @todo: move this logic
    update_chat_last_used_model(&pool, chat_id, model_name)
        .await
        .unwrap();

    let parameters = ChatCompletionParameters {
        model: model_name.to_string(),
        messages,
        ..Default::default()
    };

    let client = Client::new(api_key);

    let stream = client.chat().create_stream(parameters).await.unwrap();

    stream
}
