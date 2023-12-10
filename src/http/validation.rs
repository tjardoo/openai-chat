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
pub struct ValidationErrors {
    errors: ValidationError,
}

#[derive(Debug)]
pub enum ValidationError {
    Struct(String),
    Field(String),
}

impl std::fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "Server error".fmt(f)
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::Struct(message) => write!(f, "Struct: {}", message),
            ValidationError::Field(message) => write!(f, "Field: {}", message),
        }
    }
}

impl Error for ValidationErrors {}

impl IntoResponse for ValidationErrors {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, self.errors.to_string()).into_response()
    }
}

#[async_trait]
impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
    T: Validate,
{
    type Rejection = ValidationErrors;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(json) =
            Json::<T>::from_request(req, state)
                .await
                .map_err(|error| ValidationErrors {
                    errors: ValidationError::Struct(error.body_text().to_string()),
                })?;

        json.validate().map_err(|errors| ValidationErrors {
            errors: ValidationError::Field(
                errors
                    .errors()
                    .into_iter()
                    .map(|(key, _value)| key.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
        })?;

        Ok(ValidatedJson(json))
    }
}
