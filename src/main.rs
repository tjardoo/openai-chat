use axum::{
    extract::Request,
    http::{HeaderValue, Method},
    Router, ServiceExt,
};
use colored::*;
use dotenv::dotenv;
use std::{env, sync::Arc};
use tower::Layer;
use tower_http::normalize_path::NormalizePathLayer;
use tracing::info;

pub mod database;
pub mod http;
pub mod logging;
pub mod models;
pub mod state;

#[tokio::main]
async fn main() {
    dotenv().ok();

    logging::setup_logging().expect("Failed to set up logging");

    let db_pool = database::setup_database()
        .await
        .expect("Failed to set up database");

    let app_url = env::var("APP_URL").expect("$APP_URL is not set");
    let app_port = env::var("APP_PORT").expect("$APP_PORT is not set");

    let app_url_port = format!("{}:{}", app_url, app_port);

    let app_state = Arc::new(state::AppState { pool: db_pool });

    let middleware_wrapper = NormalizePathLayer::trim_trailing_slash();

    let router: Router<_> = Router::new()
        .nest_service("/", http::routing::public())
        .nest_service("/internal", http::routing::internal())
        .nest_service("/api", http::routing::api().with_state(app_state.clone()))
        .nest_service("/chat", http::routing::chat().with_state(app_state.clone()))
        .nest_service("/assets", http::routing::assets())
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_methods(vec![
                    Method::GET,
                    Method::POST,
                    Method::PATCH,
                    Method::DELETE,
                ])
                .allow_origin(app_url_port.parse::<HeaderValue>().unwrap())
                .allow_headers([axum::http::header::CONTENT_TYPE]),
        )
        .fallback(http::routing::fallback());

    let app = middleware_wrapper.layer(router);

    let listener = tokio::net::TcpListener::bind(&app_url_port).await.unwrap();

    info!("Server running: on [{}].", listener.local_addr().unwrap());

    println!(
        "\n{} Server running: on [{}].",
        " INFO ".on_bright_blue().white(),
        listener.local_addr().unwrap().to_string().bold()
    );

    println!(
        "\n{}",
        "Press Ctrl+C to stop the server".bright_yellow().bold()
    );

    axum::serve(listener, ServiceExt::<Request>::into_make_service(app))
        .await
        .unwrap();
}
