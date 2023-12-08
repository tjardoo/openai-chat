use askama::Template;
use axum::response::IntoResponse;

use super::templating::HtmlTemplate;

pub async fn home() -> impl IntoResponse {
    let template = HomeTemplate {};

    HtmlTemplate(template)
}

pub async fn help() -> impl IntoResponse {
    let template = HelpTemplate {};

    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

#[derive(Template)]
#[template(path = "help.html")]
struct HelpTemplate;
