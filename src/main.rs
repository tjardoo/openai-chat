use axum::Router;
use colored::*;
use dotenv::dotenv;
use std::env;
use tracing::info;

pub mod http;
pub mod logging;

#[tokio::main]
async fn main() {
    dotenv().ok();

    logging::setup_logging().expect("Failed to set up logging");

    let app_url = env::var("APP_URL").expect("$APP_URL is not set");
    let app_port = env::var("APP_PORT").expect("$APP_PORT is not set");

    let app_url_port = format!("{}:{}", app_url, app_port);

    let app = Router::new()
        .nest_service("/", http::routing::public())
        .nest_service("/api", http::routing::api())
        .nest_service("/assets", http::routing::assets())
        .fallback(http::routing::fallback());

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

    axum::serve(listener, app).await.unwrap();
}
