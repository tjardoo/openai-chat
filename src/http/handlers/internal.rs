use std::env;

use axum::{http::StatusCode, response::IntoResponse, Json};
use chrono::DateTime;
use openai_dive::v1::{api::Client, resources::model::Model};
use sqlx::{MySql, Pool};

use crate::database;

pub async fn fetch_models() -> impl IntoResponse {
    let openai_api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(openai_api_key);

    let models = client.models().list().await.unwrap();

    let pool = database::setup_database().await.unwrap();

    for model in models.data.iter() {
        upsert_model(&pool, model).await;
    }

    (StatusCode::OK, Json(models))
}

pub async fn upsert_model(pool: &Pool<MySql>, model: &Model) {
    let created_at = DateTime::from_timestamp(model.created as i64, 0).unwrap();

    sqlx::query_as!(
        Model,
        "INSERT INTO models (name, owned_by, created_at) VALUES (?, ?, ?) ON DUPLICATE KEY UPDATE owned_by = ?, created_at = ?",
        model.id,
        model.owned_by,
        created_at,
        model.owned_by,
        created_at
    )
    .execute(pool)
    .await
    .unwrap();
}
