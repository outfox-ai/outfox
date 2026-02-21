//! Videos API request and response types.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Video generation quality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VideoQuality {
    /// Quality mode (slower, better quality).
    Quality,
    /// Speed mode (faster, lower quality).
    Speed,
}

/// Video style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VideoStyle {
    /// General style.
    General,
    /// Anime style.
    Anime,
}

/// Movement amplitude.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MovementAmplitude {
    /// Auto amplitude.
    Auto,
    /// Small movement.
    Small,
    /// Medium movement.
    Medium,
    /// Large movement.
    Large,
}

/// Video task status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VideoTaskStatus {
    /// Task is processing.
    Processing,
    /// Task succeeded.
    Success,
    /// Task failed.
    Fail,
}

/// Sensitive word check configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SensitiveWordCheck {
    /// Whether to enable sensitive word check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
}

/// Image URL input for video generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VideoImageInput {
    /// Single image URL.
    Single(String),
    /// Multiple image URLs.
    Multiple(Vec<String>),
    /// Image object with URL and additional options.
    Object {
        /// Image URL.
        url: String,
        /// Additional options.
        #[serde(flatten)]
        options: Option<serde_json::Value>,
    },
}

impl From<&str> for VideoImageInput {
    fn from(s: &str) -> Self {
        VideoImageInput::Single(s.to_string())
    }
}

impl From<String> for VideoImageInput {
    fn from(s: String) -> Self {
        VideoImageInput::Single(s)
    }
}

impl From<Vec<String>> for VideoImageInput {
    fn from(v: Vec<String>) -> Self {
        VideoImageInput::Multiple(v)
    }
}

/// Video generation request.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(name = "GenerateVideoRequestArgs", setter(into, strip_option))]
pub struct GenerateVideoRequest {
    /// Model to use for video generation.
    pub model: String,
    /// Text description for video generation.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// Image(s) for video generation.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<VideoImageInput>,
    /// Output quality mode.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<VideoQuality>,
    /// Whether to include audio.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_audio: Option<bool>,
    /// Video size/resolution.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// Video duration in seconds.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// Frames per second.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fps: Option<i32>,
    /// Video style.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<VideoStyle>,
    /// Aspect ratio (e.g., "16:9", "9:16", "1:1").
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// Whether to use off-peak processing.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_peak: Option<bool>,
    /// Movement amplitude.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movement_amplitude: Option<MovementAmplitude>,
    /// Sensitive word check configuration.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive_word_check: Option<SensitiveWordCheck>,
    /// Request ID.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// User ID.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Whether to enable watermark.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watermark_enabled: Option<bool>,
}

/// Video result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoResult {
    /// Video URL.
    pub url: String,
    /// Cover image URL.
    pub cover_image_url: String,
}

/// Video generation response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoObject {
    /// Task ID for querying results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Model name.
    pub model: String,
    /// Video generation results.
    #[serde(default)]
    pub video_result: Vec<VideoResult>,
    /// Task status.
    pub task_status: VideoTaskStatus,
    /// Request ID.
    pub request_id: String,
}
