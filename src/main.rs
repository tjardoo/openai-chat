use axum::{routing::get, Router};
use colored::*;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app_url = env::var("APP_URL").expect("$APP_URL is not set");
    let app_port = env::var("APP_PORT").expect("$APP_PORT is not set");

    let app_url_port = format!("{}:{}", app_url, app_port);

    let app = Router::new().route("/", get(home));

    let listener = tokio::net::TcpListener::bind(&app_url_port).await.unwrap();

    println!(
        "\n{} Server running: on [{}].",
        " INFO ".on_bright_blue().white(),
        &app_url_port.bold()
    );

    println!(
        "\n{}",
        "Press Ctrl+C to stop the server".bright_yellow().bold()
    );

    axum::serve(listener, app).await.unwrap();
}

async fn home() -> &'static str {
    "Hello, World!"
}
