//! Tokenization request and response types.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Input for tokenization request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TokenizationInput {
    /// Single text input.
    Single(String),
    /// Multiple text inputs.
    Multiple(Vec<String>),
}

impl From<&str> for TokenizationInput {
    fn from(s: &str) -> Self {
        TokenizationInput::Single(s.to_string())
    }
}

impl From<String> for TokenizationInput {
    fn from(s: String) -> Self {
        TokenizationInput::Single(s)
    }
}

impl From<Vec<String>> for TokenizationInput {
    fn from(v: Vec<String>) -> Self {
        TokenizationInput::Multiple(v)
    }
}

impl From<Vec<&str>> for TokenizationInput {
    fn from(v: Vec<&str>) -> Self {
        TokenizationInput::Multiple(v.into_iter().map(String::from).collect())
    }
}

/// Tokenization request.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(name = "CreateTokenizationRequestArgs", setter(into, strip_option))]
pub struct CreateTokenizationRequest {
    /// Text to tokenize.
    pub text: TokenizationInput,
    /// Model ID to use.
    pub model: String,
    /// User identifier.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Tokenization result for a single input.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tokenization {
    /// Index of the input.
    pub index: i32,
    /// Object type.
    pub object: String,
    /// Total number of tokens.
    pub total_tokens: i32,
    /// Token IDs.
    pub token_ids: Vec<i32>,
    /// Offset mapping for each token.
    pub offset_mapping: Vec<Vec<i32>>,
}

/// Tokenization response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTokenizationResponse {
    /// Unique ID.
    pub id: String,
    /// Creation timestamp.
    pub created: i64,
    /// Model used.
    pub model: String,
    /// Object type.
    pub object: String,
    /// Tokenization data.
    pub data: Vec<Tokenization>,
}
