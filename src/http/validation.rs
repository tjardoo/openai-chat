use askama_axum::IntoResponse;
use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request},
    http::StatusCode,
    Json,
};
use serde::Serialize;
use std::error::Error;
use validator::Validate;

pub struct ValidatedJson<T>(pub T);

#[derive(Debug)]
pub struct ValidationErrors {
    errors: ValidationError,
}

#[derive(Serialize, Debug)]
pub enum ValidationError {
    Struct(String),
    Fields(Vec<(String, FieldError)>),
}

#[derive(Serialize, Debug)]
pub struct FieldError {
    code: String,
    params: Vec<String>,
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
            ValidationError::Fields(message) => {
                write!(f, "Field: {}", serde_json::to_string(message).unwrap())
            }
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

        json.validate()
            .map_err(|validation_errors| ValidationErrors {
                errors: ValidationError::Fields(
                    validation_errors
                        .field_errors()
                        .iter()
                        .flat_map(|(field, field_errors)| {
                            field_errors.into_iter().map(|field_error| {
                                (
                                    field.to_string(),
                                    FieldError {
                                        code: field_error.code.to_string(),
                                        params: field_error
                                            .params
                                            .iter()
                                            .map(|param| param.clone().0.to_string())
                                            .collect(),
                                    },
                                )
                            })
                        })
                        .collect(),
                ),
            })?;

        Ok(ValidatedJson(json))
    }
}
