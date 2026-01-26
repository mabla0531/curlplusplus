use reqwest::header::{InvalidHeaderName, InvalidHeaderValue};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SendRequestError {
    #[error("Invalid header name {name}: {error}")]
    InvalidHeaderName {
        name: String,
        error: InvalidHeaderName,
    },
    #[error("Invalid header value {value}: {error}")]
    InvalidHeaderValue {
        value: String,
        error: InvalidHeaderValue,
    },

    #[error("Invalid JSON body: {0}")]
    InvalidBody(#[from] serde_json::Error),

    #[error("Failed to send request: {0}")]
    ReqwestError(#[from] reqwest::Error),
}
