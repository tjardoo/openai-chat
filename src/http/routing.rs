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

pub fn internal() -> Router {
    Router::new().route("/fetch-models", get(handlers::internal::fetch_models))
}

pub fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not Found")
}

pub fn api() -> Router<Arc<AppState>> {
    let todos_router = Router::new()
        .route("/", get(handlers::api::todos::index))
        .route("/", post(handlers::api::todos::store))
        .route("/:id", get(handlers::api::todos::show))
        .route("/:id", patch(handlers::api::todos::update))
        .route("/:id", delete(handlers::api::todos::destroy));

    let models_router = Router::new().route("/", get(handlers::api::models::index));

    let chats_router = Router::new()
        .route("/", get(handlers::api::chats::index))
        .route("/", post(handlers::api::chats::store))
        .route("/:id", get(handlers::api::chats::show));

    let messages_router = Router::new()
        .route("/", get(handlers::api::messages::index))
        .route("/", post(handlers::api::messages::store))
        .route("/:id", get(handlers::api::messages::show));

    Router::new().nest(
        "/v1",
        Router::new()
            .nest("/todos", todos_router)
            .nest("/models", models_router)
            .nest(
                "/chats",
                chats_router.nest("/:id/messages", messages_router),
            ),
    )
}
