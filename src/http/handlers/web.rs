use askama::Template;
use axum::response::{Html, IntoResponse};
use sqlx::{MySql, Pool};

use crate::{database, models::model::Model};

pub async fn home() -> impl IntoResponse {
    let pool = database::setup_database().await.unwrap();

    let template = HomeTemplate {
        models: get_models(pool).await,
    };

    Html(template.render().unwrap())
}

pub async fn get_models(pool: Pool<MySql>) -> Vec<Model> {
    let models = sqlx::query_as!(
        Model,
        "SELECT
            id,
            name,
            owned_by,
            created_at
        FROM
            models"
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    models
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    models: Vec<Model>,
}
