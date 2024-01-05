use axum::http::StatusCode;
use axum::routing::{delete, get, patch, post};
use axum::Router;
use std::sync::Arc;
use tower_http::services::ServeDir;

use crate::state::AppState;

use super::handlers;

pub fn assets() -> Router {
    Router::new().nest_service(
        "/",
        ServeDir::new(format!("{}/client/dist", env!("CARGO_MANIFEST_DIR"))),
    )
}
pub fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not Found")
}

pub fn api() -> Router<Arc<AppState>> {
    let models_router = Router::new().route("/", get(handlers::api::models::index));

    let chats_router = Router::new()
        .route("/", get(handlers::api::chats::index))
        .route("/", post(handlers::api::chats::store))
        .route("/:id", get(handlers::api::chats::show))
        .route("/:id", patch(handlers::api::chats::update))
        .route("/:id", delete(handlers::api::chats::delete));

    let messages_router = Router::new()
        .route("/", get(handlers::api::messages::index))
        .route("/", post(handlers::api::messages::store))
        .route(
            "/assistant",
            post(handlers::api::messages::store_assistant_message),
        );

    Router::new().nest(
        "/v1",
        Router::new().nest("/models", models_router).nest(
            "/chats",
            chats_router.nest("/:id/messages", messages_router),
        ),
    )
}
