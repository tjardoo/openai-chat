use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request},
    http::StatusCode,
    response::IntoResponse,
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
#[serde(untagged)]
pub enum ValidationError {
    Struct { errors: Vec<FieldStructError> },
    Fields { errors: Vec<FieldError> },
}

#[derive(Serialize, Debug)]
pub struct FieldStructError {
    name: String,
    code: String,
}

#[derive(Serialize, Debug)]
pub struct FieldError {
    name: String,
    code: String,
    params: Vec<(String, String)>,
}

impl std::fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "Server error".fmt(f)
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::Struct { errors } => {
                write!(f, "{}", serde_json::to_string(errors).unwrap())
            }
            ValidationError::Fields { errors } => {
                write!(f, "{}", serde_json::to_string(errors).unwrap())
            }
        }
    }
}

impl Error for ValidationErrors {}

impl IntoResponse for ValidationErrors {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::UNPROCESSABLE_ENTITY, Json(self.errors)).into_response()
    }
}

fn extract_field_name_from_json_rejection(error: JsonRejection) -> String {
    if let JsonRejection::JsonDataError(error) = error {
        if error
            .to_string()
            .contains("Failed to deserialize the JSON body into the target type")
        {
            if error
                .source()
                .unwrap()
                .to_string()
                .contains("missing field")
            {
                let field = error.source().unwrap().to_string();

                if let (Some(start), Some(end)) = (field.find('`'), field.rfind('`')) {
                    if start < end {
                        return field[start + 1..end].to_string();
                    }
                }
            }

            if error
                .source()
                .unwrap()
                .to_string()
                .contains("invalid value")
            {
                let field = error.source().unwrap().to_string();

                return field.split(":").next().unwrap().to_string();
            }
        }

        return error.source().unwrap().to_string();
    }

    return "json error".to_string();
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
                    errors: ValidationError::Struct {
                        errors: vec![FieldStructError {
                            code: error.to_string(),
                            name: extract_field_name_from_json_rejection(error).to_string(),
                        }],
                    },
                })?;

        json.validate()
            .map_err(|validation_errors| ValidationErrors {
                errors: ValidationError::Fields {
                    errors: validation_errors
                        .field_errors()
                        .iter()
                        .flat_map(|(field, field_errors)| {
                            field_errors.into_iter().map(|field_error| FieldError {
                                name: field.to_string(),
                                code: field_error.code.to_string(),
                                params: field_error
                                    .params
                                    .iter()
                                    .filter(|param| param.0 != "value")
                                    .map(|param| {
                                        (param.clone().0.to_string(), param.clone().1.to_string())
                                    })
                                    .collect(),
                            })
                        })
                        .collect(),
                },
            })?;

        Ok(ValidatedJson(json))
    }
}
