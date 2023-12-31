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
}

#[derive(Deserialize, Validate, Debug)]
pub struct StoreAssistantMessageRequest {
    #[validate(length(min = 2, max = 2500))]
    pub content: String,
}

#[allow(dead_code)]
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
