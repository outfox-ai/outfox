//! Async task request and response types.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::ZhipuError;

/// Task status for async operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TaskStatus {
    /// Task is being processed.
    Processing,
    /// Task completed successfully.
    Success,
    /// Task failed.
    Fail,
}

/// Request to create an async chat completion.
#[derive(Clone, Default, Debug, Builder, Serialize, Deserialize)]
#[builder(name = "CreateAsyncChatRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "ZhipuError"))]
pub struct CreateAsyncChatRequest {
    /// ID of the model to use.
    pub model: String,

    /// The messages to generate chat completions for.
    pub messages: Vec<super::chat::ChatMessage>,

    /// Sampling temperature (0-2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Nucleus sampling parameter (0-1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// Maximum tokens to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    /// Stop sequences.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,

    /// Tools available to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<super::chat::Tool>>,

    /// Tool choice configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<String>,

    /// Request ID for tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,

    /// User identifier for tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// Response from async task creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsyncTaskResponse {
    /// Unique identifier for the task.
    pub id: String,
    /// Request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Model used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Task status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status: Option<TaskStatus>,
}

/// Request to create an async video generation.
#[derive(Clone, Default, Debug, Builder, Serialize, Deserialize)]
#[builder(name = "CreateAsyncVideoRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "ZhipuError"))]
pub struct CreateAsyncVideoRequest {
    /// ID of the model to use (e.g., "cogvideox-2").
    pub model: String,

    /// Text prompt for video generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    /// Reference image URL for image-to-video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,

    /// Video duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    /// Video resolution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,

    /// Video FPS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fps: Option<u32>,

    /// Request ID for tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,

    /// User identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// Request to create an async image generation.
#[derive(Clone, Default, Debug, Builder, Serialize, Deserialize)]
#[builder(name = "CreateAsyncImageRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "ZhipuError"))]
pub struct CreateAsyncImageRequest {
    /// ID of the model to use (e.g., "cogview-4").
    pub model: String,

    /// Text prompt for image generation.
    pub prompt: String,

    /// Image size (e.g., "1024x1024").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,

    /// Number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,

    /// Request ID for tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,

    /// User identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// Video result from async video generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoResult {
    /// URL of the generated video.
    pub url: String,
    /// Cover image URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_image_url: Option<String>,
}

/// Async result response for chat completion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsyncChatResult {
    /// Unique identifier.
    pub id: String,
    /// Request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Model used.
    pub model: String,
    /// Task status.
    pub task_status: TaskStatus,
    /// Chat completion choices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<super::chat::ChatChoice>>,
    /// Token usage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<super::chat::Usage>,
}

/// Async result response for video generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsyncVideoResult {
    /// Unique identifier.
    pub id: String,
    /// Request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Model used.
    pub model: String,
    /// Task status.
    pub task_status: TaskStatus,
    /// Video results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_result: Option<Vec<VideoResult>>,
}

/// Async result response for image generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsyncImageResult {
    /// Unique identifier.
    pub id: String,
    /// Request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Model used.
    pub model: String,
    /// Task status.
    pub task_status: TaskStatus,
    /// Generated image data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<super::images::ImageData>>,
}

/// Generic async result that can hold different result types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AsyncResult {
    /// Chat completion result.
    Chat(AsyncChatResult),
    /// Video generation result.
    Video(AsyncVideoResult),
    /// Image generation result.
    Image(AsyncImageResult),
}
