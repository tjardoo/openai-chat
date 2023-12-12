use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};

use crate::{
    models::model::Model,
    state::{AppState, JsonError},
};

pub async fn index(
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<Vec<Model>>), (StatusCode, Json<JsonError>)> {
    match sqlx::query_as!(
        Model,
        "SELECT
            id,
            name,
            owned_by,
            created_at
        FROM
            models"
    )
    .fetch_all(&state.pool)
    .await
    {
        Ok(models) => Ok((StatusCode::OK, Json(models))),
        Err(error) => Err((
            StatusCode::NOT_FOUND,
            Json(JsonError {
                code: StatusCode::NOT_FOUND.as_u16(),
                error: error.to_string(),
            }),
        )),
    }
}
