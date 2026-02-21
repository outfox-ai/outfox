//! Content safety/moderation request and response types.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::ZhipuError;

/// Risk level classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum RiskLevel {
    /// Content is safe.
    Pass,
    /// Content needs review.
    Review,
    /// Content violates policies.
    Reject,
}

/// Content type for moderation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ModerationInput {
    /// Text content.
    #[serde(rename = "text")]
    Text {
        /// Text to moderate (max 2000 characters).
        text: String,
    },
    /// Image content.
    #[serde(rename = "image_url")]
    Image {
        /// Image URL (max 10MB, 20x20 to 6000x6000).
        image_url: ImageUrl,
    },
    /// Video content.
    #[serde(rename = "video_url")]
    Video {
        /// Video URL (recommended 30 seconds).
        video_url: String,
    },
    /// Audio content.
    #[serde(rename = "audio_url")]
    Audio {
        /// Audio URL (recommended 60 seconds).
        audio_url: String,
    },
}

/// Image URL specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUrl {
    /// Image URL.
    pub url: String,
}

impl ModerationInput {
    /// Create text input.
    pub fn text<S: Into<String>>(text: S) -> Self {
        Self::Text { text: text.into() }
    }

    /// Create image input.
    pub fn image<S: Into<String>>(url: S) -> Self {
        Self::Image {
            image_url: ImageUrl { url: url.into() },
        }
    }

    /// Create video input.
    pub fn video<S: Into<String>>(url: S) -> Self {
        Self::Video {
            video_url: url.into(),
        }
    }

    /// Create audio input.
    pub fn audio<S: Into<String>>(url: S) -> Self {
        Self::Audio {
            audio_url: url.into(),
        }
    }
}

/// Moderation input can be text, single item, or array.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModerationInputType {
    /// Plain text string.
    Text(String),
    /// Single moderation input.
    Single(ModerationInput),
    /// Array of moderation inputs.
    Multiple(Vec<ModerationInput>),
}

impl From<String> for ModerationInputType {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for ModerationInputType {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}

impl From<ModerationInput> for ModerationInputType {
    fn from(input: ModerationInput) -> Self {
        Self::Single(input)
    }
}

impl From<Vec<ModerationInput>> for ModerationInputType {
    fn from(inputs: Vec<ModerationInput>) -> Self {
        Self::Multiple(inputs)
    }
}

/// Request for content moderation.
#[derive(Clone, Debug, Builder, Serialize, Deserialize)]
#[builder(name = "ModerationRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "ZhipuError"))]
pub struct ModerationRequest {
    /// Model to use (fixed: "moderation").
    #[builder(default = "\"moderation\".to_string()")]
    pub model: String,

    /// Content to moderate.
    pub input: ModerationInputType,

    /// Request ID for tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl Default for ModerationRequest {
    fn default() -> Self {
        Self {
            model: "moderation".to_string(),
            input: ModerationInputType::Text(String::new()),
            request_id: None,
        }
    }
}

/// A single moderation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationResult {
    /// Content type classification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// Risk level assessment.
    pub risk_level: RiskLevel,
    /// Identified risk categories.
    #[serde(default)]
    pub risk_type: Vec<String>,
}

/// Usage statistics for moderation.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModerationUsage {
    /// Number of API calls.
    pub call_count: u32,
}

/// Response from moderation API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationResponse {
    /// Unique identifier.
    pub id: String,
    /// Unix timestamp.
    pub created: u64,
    /// Request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Results for each input item.
    pub result_list: Vec<ModerationResult>,
    /// Usage statistics.
    #[serde(default)]
    pub usage: ModerationUsage,
}
