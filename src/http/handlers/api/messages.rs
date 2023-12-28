use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_streams::StreamBodyAs;
use core::iter;
use futures::{stream, Stream};
use openai_dive::v1::{
    api::Client,
    error::APIError,
    resources::chat::{ChatCompletionChunkResponse, ChatCompletionParameters, ChatMessage},
};
use std::{env, pin::Pin, sync::Arc};
use tokio_stream::StreamExt;

use crate::{
    dive::get_messages_by_chat_id,
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
            used_model,
            prompt_tokens AS \"prompt_tokens: u32\",
            completion_tokens AS \"completion_tokens: u32\",
            temperature AS \"temperature: f32\",
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

// async fn test_text_stream() -> impl IntoResponse {
//     StreamBodyAs::text(source_test_stream())
// }

fn source_test_stream() -> impl Stream<Item = String> {
    stream::iter(vec![
        "Hello".to_string(),
        "World".to_string(),
        "How".to_string(),
        "Are".to_string(),
        "You".to_string(),
    ])
    .throttle(std::time::Duration::from_secs(1))
}

type OpenAIStream =
    Pin<Box<dyn Stream<Item = Result<ChatCompletionChunkResponse, APIError>> + std::marker::Send>>;

fn source_openai_stream(stream: OpenAIStream) -> impl Stream<Item = String> {
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

// async fn source_openai_stream(mut stream: OpenAIStream) -> impl IntoResponse {
//     while let Some(response) = stream.next().await {
//         match response {
//             Ok(chat_response) => chat_response.choices.iter().for_each(|choice| {
//                 if let Some(content) = &choice.delta.content {
//                     dbg!(content);

//                     print!("{}", content);

//                     stream::iter(iter::once(content.to_string()))
//                         .throttle(std::time::Duration::from_secs(1));
//                 }
//             }),
//             Err(e) => eprintln!("{}", e),
//         }
//     }
// }

pub async fn store(
    Path(chat_id): Path<u32>,
    State(state): State<Arc<AppState>>,
    ValidatedJson(request): ValidatedJson<StoreMessageRequest>,
) -> impl IntoResponse {
    let model = request.model;

    let _last_inserted_id = sqlx::query_as!(
        Message,
        "INSERT INTO messages (chat_id, role, content, used_model) VALUES (?, ?, ?, ?)",
        chat_id,
        "user".to_string(),
        request.content,
        model
    )
    .execute(&state.pool)
    .await
    .unwrap()
    .last_insert_id();

    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let messages = get_messages_by_chat_id(&state.pool, chat_id).await.unwrap();

    let messages: Vec<ChatMessage> = messages.into_iter().map(ChatMessage::from).collect();

    let parameters = ChatCompletionParameters {
        model,
        messages,
        max_tokens: request.max_tokens,
        temperature: request.temperature,
        ..Default::default()
    };

    let client = Client::new(api_key);

    let mut stream = client.chat().create_stream(parameters).await.unwrap();

    // let mut message = String::new();

    StreamBodyAs::text(source_openai_stream(stream))
    // StreamBodyAs::text(source_openai_stream(stream))

    // let _message = crate::dive::send_message(
    //     &state.pool,
    //     chat_id,
    //     &model,
    //     request.max_tokens,
    //     request.temperature,
    // )
    // .await
    // .unwrap();

    // sqlx::query_as!(
    //     Chat,
    //     "UPDATE chats SET last_used_model = ? WHERE id = ?",
    //     model,
    //     chat_id
    // )
    // .execute(&state.pool)
    // .await
    // .unwrap();

    // match sqlx::query_as!(
    //     Message,
    //     "SELECT
    //         id,
    //         chat_id,
    //         role AS \"role: Role\",
    //         content,
    //         used_model,
    //         prompt_tokens AS \"prompt_tokens: u32\",
    //         completion_tokens AS \"completion_tokens: u32\",
    //         temperature AS \"temperature: f32\",
    //         created_at
    //     FROM
    //         messages
    //     WHERE
    //         id = ?",
    //     last_inserted_id
    // )
    // .fetch_one(&state.pool)
    // .await
    // {
    //     Ok(message) => Ok((StatusCode::CREATED, Json(message))),
    //     Err(error) => Err((
    //         StatusCode::NOT_FOUND,
    //         Json(JsonError {
    //             code: StatusCode::NOT_FOUND.as_u16(),
    //             error: error.to_string(),
    //         }),
    //     )),
    // }
}
