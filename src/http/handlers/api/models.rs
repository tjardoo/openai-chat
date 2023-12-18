use axum::{http::StatusCode, Json};
use openai_dive::v1::models::{Gpt35Engine, Gpt4Engine};

use crate::state::JsonError;

pub async fn index() -> Result<(StatusCode, Json<Vec<String>>), (StatusCode, Json<JsonError>)> {
    let mut models = Vec::<String>::new();

    models.push(Gpt4Engine::Gpt41106Preview.to_string());
    models.push(Gpt4Engine::Gpt4VisionPreview.to_string());
    models.push(Gpt4Engine::Gpt4.to_string());
    models.push(Gpt4Engine::Gpt432K.to_string());
    models.push(Gpt4Engine::Gpt40613.to_string());
    models.push(Gpt4Engine::Gpt432K0613.to_string());

    models.push(Gpt35Engine::Gpt35Turbo1106.to_string());
    models.push(Gpt35Engine::Gpt35Turbo.to_string());
    models.push(Gpt35Engine::Gpt35Turbo16K.to_string());
    models.push(Gpt35Engine::Gpt35TurboInstruct.to_string());

    Ok((StatusCode::OK, Json(models)))
}
