use axum::http::StatusCode;
use axum::routing::{delete, get, patch, post};
use axum::Router;
use std::sync::Arc;
use tower_http::services::ServeDir;

use crate::state::AppState;

use super::handlers;

pub fn public() -> Router {
    Router::new().route("/", get(handlers::web::home))
}

pub fn assets() -> Router {
    Router::new().nest_service(
        "/",
        ServeDir::new(format!("{}/assets", env!("CARGO_MANIFEST_DIR"))),
    )
}

pub fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not Found")
}

pub fn api() -> Router<Arc<AppState>> {
    Router::new().nest(
        "/v1",
        Router::new()
            .route("/todos", get(handlers::api::todos::index))
            .route("/todos", post(handlers::api::todos::store))
            .route("/todos/:id", get(handlers::api::todos::show))
            .route("/todos/:id", patch(handlers::api::todos::update))
            .route("/todos/:id", delete(handlers::api::todos::destroy)),
    )
}
