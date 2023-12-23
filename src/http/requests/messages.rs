use serde::{Deserialize, Deserializer};
use validator::Validate;

#[derive(Deserialize, Validate, Debug)]
pub struct StoreMessageRequest {
    #[validate(length(min = 2, max = 2500))]
    pub content: String,
    pub model: String,
    #[serde(default, deserialize_with = "deserialize_maybe_nan")]
    pub max_tokens: Option<u32>,
    #[validate(range(min = 0.0, max = 2.0))]
    pub temperature: Option<f32>,
}

fn deserialize_maybe_nan<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<u32>, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum InputValue {
        Number(u32),
        String(String),
    }

    let value: InputValue = Deserialize::deserialize(deserializer)?;

    match value {
        InputValue::Number(value) => Ok(Some(value)),
        InputValue::String(string) => {
            if string.is_empty() {
                Ok(None)
            } else if string.parse::<u32>().is_ok() {
                Ok(Some(string.parse::<u32>().unwrap()))
            } else {
                Err(serde::de::Error::invalid_value(
                    serde::de::Unexpected::Str(&string),
                    &"a number or empty string",
                ))
            }
        }
    }
}
