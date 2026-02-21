//! TTS request and response types.

use bytes::Bytes;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::DoubaoError;

pub mod protocol;
pub use protocol::*;

/// Audio format for TTS output.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AudioFormat {
    /// MP3 format.
    #[default]
    Mp3,
    /// PCM format.
    Pcm,
    /// WAV format.
    Wav,
    /// OGG format.
    Ogg,
}

/// Sample rate for audio output.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(into = "u32", try_from = "u32")]
pub enum SampleRate {
    /// 8000 Hz.
    Hz8000,
    /// 16000 Hz.
    Hz16000,
    /// 22050 Hz.
    Hz22050,
    /// 24000 Hz.
    #[default]
    Hz24000,
    /// 32000 Hz.
    Hz32000,
    /// 44100 Hz.
    Hz44100,
    /// 48000 Hz.
    Hz48000,
}

impl From<SampleRate> for u32 {
    fn from(rate: SampleRate) -> Self {
        match rate {
            SampleRate::Hz8000 => 8000,
            SampleRate::Hz16000 => 16000,
            SampleRate::Hz22050 => 22050,
            SampleRate::Hz24000 => 24000,
            SampleRate::Hz32000 => 32000,
            SampleRate::Hz44100 => 44100,
            SampleRate::Hz48000 => 48000,
        }
    }
}

impl TryFrom<u32> for SampleRate {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            8000 => Ok(SampleRate::Hz8000),
            16000 => Ok(SampleRate::Hz16000),
            22050 => Ok(SampleRate::Hz22050),
            24000 => Ok(SampleRate::Hz24000),
            32000 => Ok(SampleRate::Hz32000),
            44100 => Ok(SampleRate::Hz44100),
            48000 => Ok(SampleRate::Hz48000),
            _ => Err(format!("unsupported sample rate: {}", value)),
        }
    }
}

/// Audio parameters for TTS request.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(name = "AudioParamsArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "DoubaoError"))]
pub struct AudioParams {
    /// Audio output format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<AudioFormat>,

    /// Sample rate in Hz.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<u32>,

    /// Speech rate adjustment. Range: [-50, 100]. Default: 0.
    /// Negative values slow down speech, positive values speed up.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_rate: Option<i32>,

    /// Loudness adjustment. Range: [-50, 100]. 100 represents 2.0x volume, -50 represents 0.5x volume (mix voice not supported yet).
    pub loudness_rate: Option<i32>,

    /// Pitch adjustment. Range: [-12, 12]. Default: 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch_rate: Option<i32>,

    /// Enable timestamp information in response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_timestamp: Option<bool>,
}

/// TTS request parameters.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(name = "TtsRequestParamsArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "DoubaoError"))]
pub struct TtsRequestParams {
    /// Voice/speaker ID.
    pub speaker: String,

    /// Audio parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_params: Option<AudioParams>,

    /// Text to synthesize (for TaskRequest).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Additional parameters as JSON string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additions: Option<String>,
}

/// User information for session.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserInfo {
    /// User ID.
    pub uid: String,
}

/// Start session payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartSessionPayload {
    /// User information.
    pub user: UserInfo,
    /// Event type.
    pub event: i32,
    /// Namespace (always "BidirectionalTTS").
    pub namespace: String,
    /// Request parameters.
    pub req_params: TtsRequestParams,
}

/// Task request payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRequestPayload {
    /// User information.
    pub user: UserInfo,
    /// Event type.
    pub event: i32,
    /// Namespace (always "BidirectionalTTS").
    pub namespace: String,
    /// Request parameters.
    pub req_params: TtsRequestParams,
}

/// Request to create TTS speech.
#[derive(Clone, Default, Debug, Builder, PartialEq)]
#[builder(name = "CreateSpeechRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "DoubaoError"))]
pub struct CreateSpeechRequest {
    /// Text to convert to speech.
    pub text: String,

    /// Voice/speaker ID.
    pub speaker: String,

    /// Audio format.
    pub format: Option<AudioFormat>,

    /// Sample rate in Hz.
    pub sample_rate: Option<u32>,

    /// Speech rate adjustment. Range: [-50, 100]. Default: 0.
    pub speech_rate: Option<i32>,

    /// Loudness adjustment. Range: [-50, 100]. 100 represents 2.0x volume, -50 represents 0.5x volume (mix voice not supported yet).
    pub loudness_rate: Option<i32>,

    /// Pitch rate adjustment. Range: [-12, 12]. Default: 0.
    pub pitch_rate: Option<i32>,

    /// Enable timestamp information.
    pub enable_timestamp: Option<bool>,

    /// Disable markdown filter.
    pub disable_markdown_filter: Option<bool>,
}

/// TTS speech response.
#[derive(Debug, Clone)]
pub struct CreateSpeechResponse {
    /// Audio data bytes.
    pub bytes: Bytes,
    /// Audio format.
    pub format: AudioFormat,
    /// Sample rate.
    pub sample_rate: u32,
}

impl CreateSpeechResponse {
    /// Create a new speech response.
    #[must_use]
    pub fn new(bytes: Bytes, format: AudioFormat, sample_rate: u32) -> Self {
        Self {
            bytes,
            format,
            sample_rate,
        }
    }

    /// Save the audio to a file.
    #[cfg(not(target_family = "wasm"))]
    pub async fn save<P: AsRef<std::path::Path>>(
        &self,
        path: P,
    ) -> Result<(), crate::error::DoubaoError> {
        tokio::fs::write(path, &self.bytes)
            .await
            .map_err(|e| crate::error::DoubaoError::FileError(e.to_string()))
    }
}

/// Timestamp information for a word or segment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimestampInfo {
    /// Start time in milliseconds.
    pub start_ms: u64,
    /// End time in milliseconds.
    pub end_ms: u64,
    /// Text content.
    pub text: String,
}

/// Addition parameters for disabling markdown filter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Additions {
    /// Disable markdown filter in text processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_markdown_filter: Option<bool>,
}

impl Additions {
    /// Create additions with markdown filter disabled.
    #[must_use]
    pub fn new(disable_markdown_filter: bool) -> Self {
        Self {
            disable_markdown_filter: Some(disable_markdown_filter),
        }
    }

    /// Convert to JSON string.
    #[must_use]
    pub fn to_json_string(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

// =============================================================================
// V3 Unidirectional API Types (Streaming HTTP and WebSocket)
// =============================================================================

/// User information for V3 unidirectional API.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct V3UniUser {
    /// User ID.
    pub uid: String,
}

/// Audio parameters for V3 unidirectional API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct V3UniAudioParams {
    /// Audio format (mp3, ogg_opus, pcm).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// Sample rate (8000, 16000, 24000, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<u32>,

    /// Bit rate for encoded audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_rate: Option<u32>,

    /// Speech rate adjustment. Range: [-50, 100]. Default: 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_rate: Option<i32>,
}

impl Default for V3UniAudioParams {
    fn default() -> Self {
        Self {
            format: Some("mp3".to_string()),
            sample_rate: Some(24000),
            bit_rate: None,
            speech_rate: Some(0),
        }
    }
}

/// Request parameters for V3 unidirectional API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct V3UniReqParams {
    /// Text to synthesize.
    pub text: String,

    /// Voice/speaker name.
    pub speaker: String,

    /// Audio parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_params: Option<V3UniAudioParams>,
}

/// V3 unidirectional HTTP/WebSocket request body.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct V3UniRequest {
    /// User information.
    pub user: V3UniUser,

    /// Request parameters.
    pub req_params: V3UniReqParams,
}

/// V3 unidirectional streaming response (each chunk).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct V3UniStreamResponse {
    /// Response code. 0 = audio data, 20000000 = synthesis complete.
    pub code: i32,

    /// Response message.
    #[serde(default)]
    pub message: String,

    /// Base64-encoded audio data (present when code = 0).
    #[serde(default)]
    pub data: Option<String>,

    /// Usage information (present when code = 20000000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<V3UniUsage>,
}

/// Usage information in V3 response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct V3UniUsage {
    /// Number of text words/characters processed.
    pub text_words: u32,
}
