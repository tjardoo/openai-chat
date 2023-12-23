use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use serde::{Deserialize, Deserializer};
use validator::Validate;

#[derive(Deserialize, Validate, Debug)]
pub struct StoreMessageRequest {
    #[validate(length(min = 2, max = 2500))]
    pub content: String,
    pub model: String,
    #[serde(default, deserialize_with = "deserialize_with_nan")]
    pub max_tokens: Option<u32>,
    #[validate(range(min = 0.0, max = 2.0))]
    #[serde(default, deserialize_with = "deserialize_with_nan")]
    pub temperature: Option<f32>,
}

fn deserialize_with_nan<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: FromStr + Deserialize<'de> + Debug,
    T::Err: Display,
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum InputValue<T> {
        Number(T),
        String(String),
    }

    let value: InputValue<T> = Deserialize::deserialize(deserializer)?;

    match value {
        InputValue::Number(value) => Ok(Some(value)),
        InputValue::String(string) => {
            if string.is_empty() {
                Ok(None)
            } else if let Ok(parsed_value) = string.parse::<T>() {
                Ok(Some(parsed_value))
            } else {
                Err(serde::de::Error::invalid_value(
                    serde::de::Unexpected::Str(&string),
                    &"a number or empty string",
                ))
            }
        }
    }
}
