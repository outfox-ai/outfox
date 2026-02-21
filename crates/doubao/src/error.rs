//! Error types for Doubao API client.

use serde::{Deserialize, Serialize};

/// Errors that can occur when using the Doubao API client.
#[derive(Debug, thiserror::Error)]
pub enum DoubaoError {
    /// WebSocket connection error.
    #[error("websocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    /// JSON serialization/deserialization error.
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    /// Protocol error (invalid frame format, unexpected event, etc.).
    #[error("protocol error: {0}")]
    Protocol(String),

    /// API returned an error response.
    #[error("{0}")]
    ApiError(ApiError),

    /// Connection timeout.
    #[error("connection timeout")]
    Timeout,

    /// Invalid argument provided.
    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    /// Expected event not received.
    #[error("expected event {expected} not received")]
    EventNotReceived { expected: i32 },

    /// Session error.
    #[error("session error: {0}")]
    Session(String),

    /// File operation error.
    #[error("file error: {0}")]
    FileError(String),

    /// HTTP error (generic, for cases without reqwest).
    #[error("http error: {0}")]
    HttpError(String),

    /// HTTP request error (with reqwest).
    #[cfg(feature = "http")]
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    /// Configuration error.
    #[error("config error: {0}")]
    Config(String),

    /// Stream error.
    #[error("stream error: {0}")]
    Stream(String),
}

/// API error returned by Doubao service.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiError {
    /// Error code.
    pub code: Option<i32>,
    /// Error message.
    pub message: String,
    /// Additional details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(code) = self.code {
            write!(f, "[{}] {}", code, self.message)?;
        } else {
            write!(f, "{}", self.message)?;
        }
        if let Some(details) = &self.details {
            write!(f, " ({})", details)?;
        }
        Ok(())
    }
}

/// Result type alias for Doubao operations.
pub type Result<T> = std::result::Result<T, DoubaoError>;
