use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_streams::StreamBodyAs;
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
    http::{
        requests::messages::{StoreAssistantMessageRequest, StoreMessageRequest},
        validation::ValidatedJson,
    },
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

pub async fn store(
    Path(chat_id): Path<u32>,
    State(state): State<Arc<AppState>>,
    ValidatedJson(request): ValidatedJson<StoreMessageRequest>,
) -> impl IntoResponse {
    sqlx::query_as!(
        Message,
        "INSERT INTO messages (chat_id, role, content) VALUES (?, ?, ?)",
        chat_id,
        "user",
        request.content,
    )
    .execute(&state.pool)
    .await
    .unwrap()
    .last_insert_id();

    let model = request.model;

    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let messages = get_messages_by_chat_id(&state.pool, chat_id).await.unwrap();

    let messages: Vec<ChatMessage> = messages.into_iter().map(ChatMessage::from).collect();

    update_last_used_model(&state.pool, chat_id, &model)
        .await
        .unwrap();

    let parameters = ChatCompletionParameters {
        model,
        messages,
        ..Default::default()
    };

    let client = Client::new(api_key);

    let stream = client.chat().create_stream(parameters).await.unwrap();

    StreamBodyAs::text(source_openai_stream(stream))
}

pub async fn store_assistant_message(
    Path(chat_id): Path<u32>,
    State(state): State<Arc<AppState>>,
    ValidatedJson(request): ValidatedJson<StoreAssistantMessageRequest>,
) -> Result<(StatusCode, Json<Message>), (StatusCode, Json<JsonError>)> {
    let last_inserted_id = sqlx::query_as!(
        Message,
        "INSERT INTO messages (chat_id, role, content) VALUES (?, ?, ?)",
        chat_id,
        "assistant",
        request.content,
    )
    .execute(&state.pool)
    .await
    .unwrap()
    .last_insert_id();

    let message = sqlx::query_as!(
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
    .unwrap();

    Ok((StatusCode::CREATED, Json(message)))
}

async fn update_last_used_model(
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
