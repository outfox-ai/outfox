//! Image generation request and response types.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Response format for generated images.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ImageResponseFormat {
    /// Base64 encoded JSON.
    #[default]
    B64Json,
    /// URL to the image.
    Url,
}

/// Size option for generated images.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ImageSize {
    /// Adaptive size.
    #[default]
    Adaptive,
}

/// Optimize prompt thinking mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum OptimizePromptThinking {
    /// Auto mode.
    #[default]
    Auto,
    /// Enabled mode.
    Enabled,
    /// Disabled mode.
    Disabled,
}

/// Optimize prompt mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum OptimizePromptMode {
    /// Standard mode.
    #[default]
    Standard,
    /// Fast mode.
    Fast,
}

/// Options for prompt optimization.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OptimizePromptOptions {
    /// Thinking mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<OptimizePromptThinking>,
    /// Optimization mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<OptimizePromptMode>,
}

/// Sequential image generation mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SequentialImageGeneration {
    /// Auto mode.
    #[default]
    Auto,
    /// Disabled mode.
    Disabled,
}

/// Options for sequential image generation.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SequentialImageGenerationOptions {
    /// Maximum number of images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_images: Option<i32>,
}

/// Image input for image-to-image generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImageInput {
    /// Single image URL.
    Single(String),
    /// Multiple image URLs.
    Multiple(Vec<String>),
}

impl From<&str> for ImageInput {
    fn from(s: &str) -> Self {
        ImageInput::Single(s.to_string())
    }
}

impl From<String> for ImageInput {
    fn from(s: String) -> Self {
        ImageInput::Single(s)
    }
}

impl From<Vec<String>> for ImageInput {
    fn from(v: Vec<String>) -> Self {
        ImageInput::Multiple(v)
    }
}

/// Image generation request.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(name = "GenerateImagesRequestArgs", setter(into, strip_option))]
pub struct GenerateImagesRequest {
    /// Model ID to use.
    pub model: String,
    /// Text prompt for image generation.
    pub prompt: String,
    /// Input image(s) for image-to-image generation.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ImageInput>,
    /// Response format.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ImageResponseFormat>,
    /// Random seed for generation.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// Guidance scale.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// Size of generated images.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// Whether to add watermark.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watermark: Option<bool>,
    /// Whether to optimize the prompt.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimize_prompt: Option<bool>,
    /// Options for prompt optimization.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimize_prompt_options: Option<OptimizePromptOptions>,
    /// Sequential image generation mode.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequential_image_generation: Option<SequentialImageGeneration>,
    /// Options for sequential image generation.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequential_image_generation_options: Option<SequentialImageGenerationOptions>,
}

/// Generated image.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    /// URL of the image (if response_format is url).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Base64 encoded image (if response_format is b64_json).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b64_json: Option<String>,
    /// Size of the image.
    pub size: String,
}

/// Usage information for image generation.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GenerateImagesUsage {
    /// Number of images generated.
    pub generated_images: i64,
    /// Number of output tokens.
    pub output_tokens: i64,
    /// Total tokens.
    pub total_tokens: i64,
}

/// Error in image generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateImagesError {
    /// Error code.
    pub code: String,
    /// Error message.
    pub message: String,
}

/// Image generation response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateImagesResponse {
    /// Model used.
    pub model: String,
    /// Creation timestamp.
    pub created: i64,
    /// Generated images.
    pub data: Vec<Image>,
    /// Usage information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<GenerateImagesUsage>,
    /// Error information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<GenerateImagesError>,
}

/// Image generation stream event types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageGenerationStreamEventType {
    /// Partial images generated successfully.
    #[serde(rename = "image_generation.partial_succeeded")]
    PartialSucceeded,
    /// Partial image generation failed.
    #[serde(rename = "image_generation.partial_failed")]
    PartialFailed,
    /// Image generation completed.
    #[serde(rename = "image_generation.completed")]
    Completed,
}

/// Streaming image generation response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateImagesStreamResponse {
    /// Event type.
    #[serde(rename = "type")]
    pub event_type: ImageGenerationStreamEventType,
    /// Model used.
    pub model: String,
    /// Creation timestamp.
    pub created: i64,
    /// Index of the image.
    pub image_index: i64,
    /// URL of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Base64 encoded image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b64_json: Option<String>,
    /// Size of the image.
    pub size: String,
    /// Usage information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<GenerateImagesUsage>,
    /// Error information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<GenerateImagesError>,
}
