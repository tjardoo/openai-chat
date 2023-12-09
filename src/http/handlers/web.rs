use askama::Template;
use axum::response::{Html, IntoResponse};

pub async fn home() -> impl IntoResponse {
    let template = HomeTemplate {};

    Html(template.render().unwrap())
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;
