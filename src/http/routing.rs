use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use tower_http::services::ServeDir;

use super::handlers;

pub fn public() -> Router {
    Router::new()
        .route("/", get(handlers::home))
        .route("/help", get(handlers::help))
}

pub fn api() -> Router {
    Router::new().nest("/v1", Router::new().route("/hello", get(handlers::api)))
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
