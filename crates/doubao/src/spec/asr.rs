//! ASR request and response types.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::DoubaoError;

pub mod protocol;
pub use protocol::*;

/// Audio format for ASR input.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AsrAudioFormat {
    /// MP3 format.
    #[default]
    Mp3,
    /// WAV format.
    Wav,
    /// Raw PCM format.
    Raw,
    /// OGG format.
    Ogg,
}

/// Audio codec.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AudioCodec {
    /// Raw PCM.
    #[default]
    Raw,
    /// Opus codec.
    Opus,
}

/// User information for ASR request.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AsrUserInfo {
    /// User ID.
    pub uid: String,
}

/// Audio configuration for ASR request.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(name = "AsrAudioConfigArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "DoubaoError"))]
pub struct AsrAudioConfig {
    /// Audio container format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<AsrAudioFormat>,

    /// Audio URL (mutually exclusive with data).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Base64 encoded audio data (mutually exclusive with url).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    /// Language code. Empty supports Chinese, English, Shanghai dialect, Hokkien, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Audio codec (for raw format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<AudioCodec>,

    /// Sample rate in Hz (default: 16000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<u32>,

    /// Bits per sample (default: 16).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bits: Option<u8>,

    /// Number of channels (default: 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<u8>,
}

/// Request configuration for ASR.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(name = "AsrRequestConfigArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "DoubaoError"))]
pub struct AsrRequestConfig {
    /// Model name (must be "bigmodel").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,

    /// Enable inverse text normalization (e.g., "一百" -> "100"). Default: true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_itn: Option<bool>,

    /// Enable punctuation. Default: false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_punc: Option<bool>,

    /// Enable speaker diarization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_speaker_info: Option<bool>,

    /// Model version. "400" for new model, empty for 310 default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_version: Option<String>,

    /// Show intermediate results (for streaming).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_utterances: Option<bool>,

    /// Result type (for streaming): "single" or "full".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_type: Option<String>,
}

/// Submit task request body (standard version).
#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(name = "SubmitTaskRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "DoubaoError"))]
pub struct SubmitTaskRequest {
    /// User information.
    pub user: AsrUserInfo,

    /// Audio configuration.
    pub audio: AsrAudioConfig,

    /// Request configuration.
    pub request: AsrRequestConfig,

    /// Callback URL for result notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<String>,

    /// Custom data to include in callback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,
}

/// Flash recognition request body (turbo version).
#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(name = "FlashRecognizeRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "DoubaoError"))]
pub struct FlashRecognizeRequest {
    /// User information.
    pub user: AsrUserInfo,

    /// Audio configuration.
    pub audio: AsrAudioConfig,

    /// Request configuration.
    pub request: AsrRequestConfig,
}

/// Word-level recognition result.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AsrWord {
    /// Recognized text.
    pub text: String,

    /// Start time in milliseconds.
    pub start_time: i32,

    /// End time in milliseconds.
    pub end_time: i32,

    /// Confidence score (0-1).
    #[serde(default)]
    pub confidence: f32,
}

/// Utterance (sentence) recognition result.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AsrUtterance {
    /// Recognized text.
    pub text: String,

    /// Start time in milliseconds.
    #[serde(default)]
    pub start_time: i32,

    /// End time in milliseconds.
    #[serde(default)]
    pub end_time: i32,

    /// Word-level results.
    #[serde(default)]
    pub words: Vec<AsrWord>,

    /// Speaker ID (if speaker diarization is enabled).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speaker: Option<String>,

    /// Whether this is a final result (for streaming).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definite: Option<bool>,
}

/// Recognition result.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AsrResult {
    /// Full transcribed text.
    #[serde(default)]
    pub text: String,

    /// Utterance-level results.
    #[serde(default)]
    pub utterances: Vec<AsrUtterance>,

    /// Additional information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additions: Option<serde_json::Value>,
}

/// Audio information from response.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AsrAudioInfo {
    /// Audio duration in milliseconds.
    #[serde(default)]
    pub duration: i32,
}

/// Response body for recognition APIs.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AsrResponse {
    /// Recognition result.
    #[serde(default)]
    pub result: AsrResult,

    /// Audio information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_info: Option<AsrAudioInfo>,
}

/// Task status from query response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    /// Task completed successfully.
    Success,
    /// Task is being processed.
    Processing,
    /// Task is waiting in queue.
    InQueue,
    /// Audio is silent.
    Silent,
    /// Error occurred.
    Error(i32),
}

impl TaskStatus {
    /// Create TaskStatus from status code.
    #[must_use]
    pub fn from_code(code: i32) -> Self {
        use self::protocol::*;
        match code {
            STATUS_SUCCESS => Self::Success,
            STATUS_PROCESSING => Self::Processing,
            STATUS_IN_QUEUE => Self::InQueue,
            STATUS_SILENT => Self::Silent,
            _ => Self::Error(code),
        }
    }

    /// Check if the task is still pending (processing or in queue).
    #[must_use]
    pub fn is_pending(&self) -> bool {
        matches!(self, Self::Processing | Self::InQueue)
    }

    /// Check if the task completed (success or silent).
    #[must_use]
    pub fn is_completed(&self) -> bool {
        matches!(self, Self::Success | Self::Silent)
    }

    /// Check if the task failed.
    #[must_use]
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error(_))
    }
}

/// Query response with status.
#[derive(Debug, Clone)]
pub struct QueryResponse {
    /// Task status.
    pub status: TaskStatus,
    /// Status message.
    pub message: String,
    /// Log ID for debugging.
    pub log_id: Option<String>,
    /// Recognition result (if available).
    pub result: Option<AsrResponse>,
}

/// Streaming ASR session configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(name = "StreamingAsrConfigArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "DoubaoError"))]
pub struct StreamingAsrConfig {
    /// Audio format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<AsrAudioFormat>,

    /// Audio codec.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<AudioCodec>,

    /// Sample rate in Hz.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<u32>,

    /// Bits per sample.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bits: Option<u8>,

    /// Number of channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<u8>,

    /// Language code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Enable ITN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_itn: Option<bool>,

    /// Enable punctuation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_punc: Option<bool>,

    /// Show intermediate results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_utterances: Option<bool>,

    /// Result type: "single" or "full".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_type: Option<String>,
}

/// Streaming ASR result event.
#[derive(Debug, Clone)]
pub struct StreamingAsrResult {
    /// Session ID.
    pub session_id: String,
    /// Recognition result.
    pub result: AsrResult,
    /// Whether this is the final result.
    pub is_final: bool,
}
