//! Embeddings request and response types.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::ZhipuError;

/// Request to create embeddings.
#[derive(Clone, Default, Debug, Builder, Serialize, Deserialize)]
#[builder(name = "CreateEmbeddingsRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "ZhipuError"))]
pub struct CreateEmbeddingsRequest {
    /// ID of the model to use.
    pub model: String,

    /// Input text to embed. Can be a string or array of strings.
    pub input: EmbeddingInput,
}

/// Input for embeddings request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmbeddingInput {
    /// Single string input.
    Single(String),
    /// Multiple string inputs.
    Multiple(Vec<String>),
}

impl Default for EmbeddingInput {
    fn default() -> Self {
        Self::Single(String::new())
    }
}

impl From<String> for EmbeddingInput {
    fn from(s: String) -> Self {
        Self::Single(s)
    }
}

impl From<&str> for EmbeddingInput {
    fn from(s: &str) -> Self {
        Self::Single(s.to_string())
    }
}

impl From<Vec<String>> for EmbeddingInput {
    fn from(v: Vec<String>) -> Self {
        Self::Multiple(v)
    }
}

impl From<Vec<&str>> for EmbeddingInput {
    fn from(v: Vec<&str>) -> Self {
        Self::Multiple(v.into_iter().map(String::from).collect())
    }
}

/// An embedding object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embedding {
    /// Object type (always "embedding").
    pub object: String,
    /// The embedding vector.
    pub embedding: Vec<f32>,
    /// The index of this embedding.
    pub index: u32,
}

/// Usage statistics for embeddings.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmbeddingUsage {
    /// Number of tokens in the prompt.
    pub prompt_tokens: u32,
    /// Total number of tokens.
    pub total_tokens: u32,
}

/// Response from the embeddings API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEmbeddingsResponse {
    /// Object type (always "list").
    pub object: String,
    /// List of embedding objects.
    pub data: Vec<Embedding>,
    /// Model used for the embeddings.
    pub model: String,
    /// Token usage statistics.
    pub usage: EmbeddingUsage,
}

/// Available embedding models.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmbeddingModel {
    /// Embedding-2 model.
    Embedding2,
    /// Embedding-3 model.
    Embedding3,
}

impl EmbeddingModel {
    /// Get the model ID string.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Embedding2 => "embedding-2",
            Self::Embedding3 => "embedding-3",
        }
    }
}

impl std::fmt::Display for EmbeddingModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<EmbeddingModel> for String {
    fn from(model: EmbeddingModel) -> Self {
        model.as_str().to_string()
    }
}
