use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;
use tower_http::services::ServeDir;

use crate::state::AppState;

use super::handlers;

pub fn public() -> Router {
    Router::new()
        .route("/", get(handlers::home))
        .route("/help", get(handlers::help))
}

pub fn api() -> Router<Arc<AppState>> {
    Router::new().nest(
        "/v1",
        Router::new()
            .route("/todos", post(handlers::index_todo))
            .route("/todos/:id", get(handlers::show_todo)),
    )
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
