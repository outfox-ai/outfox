//! Speech-to-text (ASR) request and response types.

use bytes::Bytes;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::ZhipuError;

/// Available ASR models.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AsrModel {
    /// GLM-ASR-2512 - Standard ASR model.
    #[default]
    GlmAsr2512,
}

impl AsrModel {
    /// Get the model ID string.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::GlmAsr2512 => "glm-asr-2512",
        }
    }
}

impl std::fmt::Display for AsrModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<AsrModel> for String {
    fn from(model: AsrModel) -> Self {
        model.as_str().to_string()
    }
}

impl Serialize for AsrModel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for AsrModel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "glm-asr-2512" => Ok(Self::GlmAsr2512),
            _ => Err(serde::de::Error::unknown_variant(&s, &["glm-asr-2512"])),
        }
    }
}

/// Audio input source for ASR.
#[derive(Debug, Clone)]
pub enum AudioInput {
    /// Audio file as bytes with filename.
    File { data: Bytes, filename: String },
    /// Base64-encoded audio data.
    Base64(String),
}

impl AudioInput {
    /// Create from file bytes.
    pub fn from_bytes<B: Into<Bytes>, S: Into<String>>(data: B, filename: S) -> Self {
        Self::File {
            data: data.into(),
            filename: filename.into(),
        }
    }

    /// Create from base64 string.
    pub fn from_base64<S: Into<String>>(base64: S) -> Self {
        Self::Base64(base64.into())
    }

    /// Read audio from a file path.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read.
    pub async fn from_path<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Self> {
        let path = path.as_ref();
        let data = tokio::fs::read(path).await?;
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("audio.wav")
            .to_string();
        Ok(Self::File {
            data: Bytes::from(data),
            filename,
        })
    }
}

/// Request to transcribe audio to text.
#[derive(Clone, Default, Debug, Builder)]
#[builder(name = "CreateTranscriptionRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "ZhipuError"))]
pub struct CreateTranscriptionRequest {
    /// The audio input (file or base64).
    #[builder(setter(custom))]
    pub audio: Option<AudioInput>,

    /// ID of the model to use (glm-asr-2512).
    #[builder(default = "AsrModel::GlmAsr2512")]
    pub model: AsrModel,

    /// Previous transcription context for longer audio (recommend <8000 chars).
    pub prompt: Option<String>,

    /// Domain vocabulary list to improve recognition (max 100 items).
    pub hotwords: Option<Vec<String>>,

    /// Enable streaming responses via Server-Sent Events.
    pub stream: Option<bool>,

    /// Unique request identifier.
    pub request_id: Option<String>,

    /// End-user ID for abuse monitoring (6-128 characters).
    pub user_id: Option<String>,
}

impl CreateTranscriptionRequestArgs {
    /// Set the audio input.
    pub fn audio(&mut self, audio: AudioInput) -> &mut Self {
        self.audio = Some(Some(audio));
        self
    }

    /// Set audio from bytes.
    pub fn audio_bytes<B: Into<Bytes>, S: Into<String>>(
        &mut self,
        data: B,
        filename: S,
    ) -> &mut Self {
        self.audio(AudioInput::from_bytes(data, filename))
    }

    /// Set audio from base64 string.
    pub fn audio_base64<S: Into<String>>(&mut self, base64: S) -> &mut Self {
        self.audio(AudioInput::from_base64(base64))
    }
}

/// Response from the transcription API (non-streaming).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionResponse {
    /// Unique identifier for the transcription.
    pub id: String,
    /// Unix timestamp of creation.
    pub created: u64,
    /// Request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Model used for transcription.
    pub model: String,
    /// The transcribed text.
    pub text: String,
}

/// Event type for streaming transcription.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TranscriptionEventType {
    /// Partial text delta.
    #[serde(rename = "transcript.text.delta")]
    TextDelta,
    /// Final text (done).
    #[serde(rename = "transcript.text.done")]
    TextDone,
}

/// Streaming chunk response for ASR.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionStreamChunk {
    /// Unique identifier for the transcription.
    pub id: String,
    /// Unix timestamp of creation.
    pub created: u64,
    /// Model used for transcription.
    pub model: String,
    /// Event type (delta or done).
    #[serde(rename = "type")]
    pub event_type: TranscriptionEventType,
    /// Text delta/content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<String>,
}
