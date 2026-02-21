//! Text tokenizer request and response types.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::ZhipuError;

/// Request to tokenize text.
#[derive(Clone, Default, Debug, Builder, Serialize, Deserialize)]
#[builder(name = "TokenizerRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "ZhipuError"))]
pub struct TokenizerRequest {
    /// Model identifier.
    pub model: String,

    /// Messages to tokenize.
    pub messages: Vec<super::chat::ChatMessage>,

    /// Tools to include in token count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<super::chat::Tool>>,

    /// Request ID for tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,

    /// User identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// Token usage breakdown.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TokenizerUsage {
    /// Tokens in prompt/messages.
    pub prompt_tokens: u32,
    /// Tokens for video content.
    #[serde(default)]
    pub video_tokens: u32,
    /// Tokens for image content.
    #[serde(default)]
    pub image_tokens: u32,
    /// Total token count.
    pub total_tokens: u32,
}

/// Response from tokenizer API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenizerResponse {
    /// Unique identifier.
    pub id: String,
    /// Request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Unix timestamp.
    pub created: u64,
    /// Token usage breakdown.
    pub usage: TokenizerUsage,
}
