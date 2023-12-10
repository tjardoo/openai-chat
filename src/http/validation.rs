use askama_axum::IntoResponse;
use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request},
    http::StatusCode,
    Json,
};
use std::error::Error;
use validator::Validate;

pub struct ValidatedJson<T>(pub T);

#[derive(Debug)]
pub struct ValidationError;

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "Server error".fmt(f)
    }
}

impl Error for ValidationError {}

impl IntoResponse for ValidationError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, "Incorrect JSON").into_response()
    }
}

#[async_trait]
impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
    T: Validate,
{
    type Rejection = ValidationError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(json) = Json::<T>::from_request(req, state)
            .await
            .map_err(|_| ValidationError)?;

        json.validate().map_err(|_| ValidationError)?;

        Ok(ValidatedJson(json))
    }
}
