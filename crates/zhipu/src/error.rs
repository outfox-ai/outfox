//! Error types for Zhipu AI API client.

use serde::{Deserialize, Serialize};

/// Errors that can occur when using the Zhipu AI API client.
#[derive(Debug, thiserror::Error)]
pub enum ZhipuError {
    /// HTTP request error.
    #[error("http error: {0}")]
    Reqwest(#[from] reqwest::Error),

    /// JSON serialization/deserialization error.
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    /// API returned an error response.
    #[error("{0}")]
    ApiError(ApiError),

    /// Stream error.
    #[error("stream error: {0}")]
    Stream(String),

    /// Invalid argument provided.
    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    /// File operation error.
    #[error("file error: {0}")]
    FileError(String),
}

/// API error returned by Zhipu AI service.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiError {
    /// Error code.
    #[serde(default)]
    pub code: Option<String>,
    /// Error message.
    pub message: String,
    /// Error type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Parameter that caused the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(code) = &self.code {
            write!(f, "[{}] ", code)?;
        }
        if let Some(kind) = &self.kind {
            write!(f, "{}: ", kind)?;
        }
        write!(f, "{}", self.message)?;
        if let Some(param) = &self.param {
            write!(f, " (param: {})", param)?;
        }
        Ok(())
    }
}

/// Wrapper for API error response.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorResponse {
    /// The error object.
    pub error: ApiError,
}

/// Result type alias for Zhipu operations.
pub type Result<T> = std::result::Result<T, ZhipuError>;
