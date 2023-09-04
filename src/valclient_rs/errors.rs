use json::JsonError;
use reqwest::header::InvalidHeaderValue;
use std::fmt::{write, Debug, Display, Formatter};

pub struct ValClientError {
    pub data: String,
    pub(crate) debug: String,
}

impl ValClientError {
    pub fn new(data_str: &str, debug_str: &str) -> Self {
        Self {
            data: data_str.to_owned(),
            debug: debug_str.to_owned(),
        }
    }
}

impl Display for ValClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl Debug for ValClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.data, self.debug)
    }
}

impl std::error::Error for ValClientError {}

impl From<reqwest::Error> for ValClientError {
    fn from(error: reqwest::Error) -> Self {
        Self {
            data: "Request failed".to_owned(),
            debug: error.to_string(),
        }
    }
}

impl From<base64::DecodeError> for ValClientError {
    fn from(error: base64::DecodeError) -> Self {
        Self {
            data: "Failed to decode base64".to_owned(),
            debug: error.to_string(),
        }
    }
}

impl From<std::io::Error> for ValClientError {
    fn from(error: std::io::Error) -> Self {
        Self {
            data: "Failed to read lockfile".to_owned(),
            debug: error.to_string(),
        }
    }
}

impl From<JsonError> for ValClientError {
    fn from(error: JsonError) -> Self {
        Self {
            data: "Failed to parse json".to_owned(),
            debug: error.to_string(),
        }
    }
}

impl From<InvalidHeaderValue> for ValClientError {
    fn from(error: InvalidHeaderValue) -> Self {
        Self {
            data: "Failed to parse header value".to_owned(),
            debug: error.to_string(),
        }
    }
}
