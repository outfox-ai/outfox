//! Embeddings request and response types.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Encoding format for embeddings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum EmbeddingEncodingFormat {
    /// Float format (default).
    #[default]
    Float,
    /// Base64 encoded format.
    Base64,
}

/// Embedding request for text inputs.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(name = "CreateEmbeddingRequestArgs", setter(into, strip_option))]
pub struct CreateEmbeddingRequest {
    /// Input text(s) to embed.
    pub input: EmbeddingInput,
    /// Model ID to use.
    pub model: String,
    /// User identifier.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Encoding format for the embeddings.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<EmbeddingEncodingFormat>,
    /// Number of dimensions for the embeddings.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<i32>,
}

/// Input for embedding request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmbeddingInput {
    /// Single text input.
    Single(String),
    /// Multiple text inputs.
    Multiple(Vec<String>),
    /// Token inputs.
    Tokens(Vec<Vec<i32>>),
}

impl From<&str> for EmbeddingInput {
    fn from(s: &str) -> Self {
        EmbeddingInput::Single(s.to_string())
    }
}

impl From<String> for EmbeddingInput {
    fn from(s: String) -> Self {
        EmbeddingInput::Single(s)
    }
}

impl From<Vec<String>> for EmbeddingInput {
    fn from(v: Vec<String>) -> Self {
        EmbeddingInput::Multiple(v)
    }
}

impl From<Vec<&str>> for EmbeddingInput {
    fn from(v: Vec<&str>) -> Self {
        EmbeddingInput::Multiple(v.into_iter().map(String::from).collect())
    }
}

/// Single embedding result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embedding {
    /// Object type.
    pub object: String,
    /// The embedding vector.
    pub embedding: Vec<f32>,
    /// Index of the input.
    pub index: i32,
}

/// Embedding response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEmbeddingResponse {
    /// Unique ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Object type.
    pub object: String,
    /// Creation timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    /// Model used.
    pub model: String,
    /// Embedding data.
    pub data: Vec<Embedding>,
    /// Token usage.
    pub usage: EmbeddingUsage,
}

/// Token usage for embeddings.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmbeddingUsage {
    /// Number of tokens in the prompt.
    pub prompt_tokens: i64,
    /// Total tokens used.
    pub total_tokens: i64,
}

// --- Multimodal Embeddings ---

/// Input type for multimodal embeddings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MultimodalEmbeddingInputType {
    /// Text input.
    Text,
    /// Image URL input.
    ImageUrl,
    /// Video URL input.
    VideoUrl,
}

/// Image URL for multimodal embeddings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultimodalEmbeddingImageUrl {
    /// URL of the image.
    pub url: String,
}

/// Video URL for multimodal embeddings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultimodalEmbeddingVideoUrl {
    /// URL of the video.
    pub url: String,
    /// Frames per second.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fps: Option<f64>,
}

/// Input item for multimodal embeddings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultimodalEmbeddingInput {
    /// Type of input.
    #[serde(rename = "type")]
    pub input_type: MultimodalEmbeddingInputType,
    /// Text content (if type is text).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Image URL (if type is image_url).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<MultimodalEmbeddingImageUrl>,
    /// Video URL (if type is video_url).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_url: Option<MultimodalEmbeddingVideoUrl>,
}

impl MultimodalEmbeddingInput {
    /// Create a text input.
    #[must_use]
    pub fn text<S: Into<String>>(text: S) -> Self {
        Self {
            input_type: MultimodalEmbeddingInputType::Text,
            text: Some(text.into()),
            image_url: None,
            video_url: None,
        }
    }

    /// Create an image URL input.
    #[must_use]
    pub fn image_url<S: Into<String>>(url: S) -> Self {
        Self {
            input_type: MultimodalEmbeddingInputType::ImageUrl,
            text: None,
            image_url: Some(MultimodalEmbeddingImageUrl { url: url.into() }),
            video_url: None,
        }
    }

    /// Create a video URL input.
    #[must_use]
    pub fn video_url<S: Into<String>>(url: S) -> Self {
        Self {
            input_type: MultimodalEmbeddingInputType::VideoUrl,
            text: None,
            image_url: None,
            video_url: Some(MultimodalEmbeddingVideoUrl {
                url: url.into(),
                fps: None,
            }),
        }
    }
}

/// Sparse embedding input type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SparseEmbeddingInputType {
    /// Enable sparse embeddings.
    Enabled,
    /// Disable sparse embeddings.
    Disabled,
}

/// Sparse embedding input configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SparseEmbeddingInput {
    /// Type of sparse embedding.
    #[serde(rename = "type")]
    pub input_type: SparseEmbeddingInputType,
}

/// Multimodal embedding request.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(
    name = "CreateMultimodalEmbeddingRequestArgs",
    setter(into, strip_option)
)]
pub struct CreateMultimodalEmbeddingRequest {
    /// Input items.
    pub input: Vec<MultimodalEmbeddingInput>,
    /// Model ID to use.
    pub model: String,
    /// Encoding format.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<EmbeddingEncodingFormat>,
    /// Number of dimensions.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<i32>,
    /// Sparse embedding configuration.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sparse_embedding: Option<SparseEmbeddingInput>,
}

/// Sparse embedding entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SparseEmbedding {
    /// Index of the sparse embedding.
    pub index: i32,
    /// Value of the sparse embedding.
    pub value: f64,
}

/// Multimodal embedding result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultimodalEmbedding {
    /// Object type.
    pub object: String,
    /// The embedding vector.
    pub embedding: Vec<f32>,
    /// Sparse embeddings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sparse_embedding: Option<Vec<SparseEmbedding>>,
}

/// Token details for multimodal embeddings.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MultimodalEmbeddingPromptTokensDetail {
    /// Text tokens.
    pub text_tokens: i32,
    /// Image tokens.
    pub image_tokens: i32,
}

/// Usage for multimodal embeddings.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MultimodalEmbeddingUsage {
    /// Number of prompt tokens.
    pub prompt_tokens: i32,
    /// Total tokens.
    pub total_tokens: i32,
    /// Token details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_tokens_details: Option<MultimodalEmbeddingPromptTokensDetail>,
}

/// Multimodal embedding response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMultimodalEmbeddingResponse {
    /// Unique ID.
    pub id: String,
    /// Object type.
    pub object: String,
    /// Creation timestamp.
    pub created: i64,
    /// Model used.
    pub model: String,
    /// Embedding data.
    pub data: MultimodalEmbedding,
    /// Token usage.
    pub usage: MultimodalEmbeddingUsage,
}
