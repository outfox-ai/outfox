//! Text reranking request and response types.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::ZhipuError;

/// Request to rerank documents.
#[derive(Clone, Default, Debug, Builder, Serialize, Deserialize)]
#[builder(name = "RerankRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "ZhipuError"))]
pub struct RerankRequest {
    /// Model identifier (default: "rerank").
    #[builder(default = "\"rerank\".to_string()")]
    pub model: String,

    /// Search text for matching (max 4096 characters).
    pub query: String,

    /// Candidate texts to score (max 128 items, 4096 chars each).
    pub documents: Vec<String>,

    /// Return top n results (0 returns all).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_n: Option<u32>,

    /// Include original text in response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_documents: Option<bool>,

    /// Include raw scores in response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_raw_scores: Option<bool>,

    /// Request ID for tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,

    /// User identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// A single reranking result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RerankResult {
    /// Original document text (if requested).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    /// Original position in input array.
    pub index: u32,
    /// Relevance score.
    pub relevance_score: f64,
}

/// Token usage for reranking.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RerankUsage {
    /// Tokens in prompt.
    pub prompt_tokens: u32,
    /// Total tokens used.
    pub total_tokens: u32,
}

/// Response from reranking API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RerankResponse {
    /// Unique identifier.
    pub id: String,
    /// Unix timestamp.
    pub created: u64,
    /// Request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Reranked results sorted by relevance.
    pub results: Vec<RerankResult>,
    /// Token usage.
    #[serde(default)]
    pub usage: RerankUsage,
}
