//! Text-to-speech request and response types.

use bytes::Bytes;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::ZhipuError;

/// Available TTS voices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Voice {
    /// Tongtong - default voice.
    #[default]
    Tongtong,
    /// Chuichui voice.
    Chuichui,
    /// Xiaochen voice.
    Xiaochen,
    /// Jam voice.
    Jam,
    /// Kazi voice.
    Kazi,
    /// Douji voice.
    Douji,
    /// Luodo voice.
    Luodo,
}

impl Voice {
    /// Get the voice ID string.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Tongtong => "彤彤",
            Self::Chuichui => "锤锤",
            Self::Xiaochen => "小陈",
            Self::Jam => "jam",
            Self::Kazi => "kazi",
            Self::Douji => "douji",
            Self::Luodo => "luodo",
        }
    }
}

impl std::fmt::Display for Voice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Output audio format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AudioFormat {
    /// WAV format.
    #[default]
    Wav,
    /// PCM format (raw audio).
    Pcm,
}

impl AudioFormat {
    /// Get the format string.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Wav => "wav",
            Self::Pcm => "pcm",
        }
    }
}

/// Stream encoding format for streaming mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StreamEncoding {
    /// Base64 encoding.
    #[default]
    Base64,
    /// Hex encoding.
    Hex,
}

/// Available TTS models.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TtsModel {
    /// GLM-TTS - Standard TTS model.
    #[default]
    GlmTts,
}

impl TtsModel {
    /// Get the model ID string.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::GlmTts => "glm-tts",
        }
    }
}

impl std::fmt::Display for TtsModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<TtsModel> for String {
    fn from(model: TtsModel) -> Self {
        model.as_str().to_string()
    }
}

impl Serialize for TtsModel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for TtsModel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "glm-tts" => Ok(Self::GlmTts),
            _ => Err(serde::de::Error::unknown_variant(&s, &["glm-tts"])),
        }
    }
}

/// Request to create speech from text.
#[derive(Clone, Default, Debug, Builder, Serialize, Deserialize)]
#[builder(name = "CreateSpeechRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "ZhipuError"))]
pub struct CreateSpeechRequest {
    /// ID of the model to use (glm-tts).
    #[builder(default = "TtsModel::GlmTts")]
    pub model: TtsModel,

    /// The text to convert to speech (max 1024 characters).
    pub input: String,

    /// The voice to use for synthesis.
    #[builder(default = "Voice::Tongtong")]
    pub voice: Voice,

    /// Enable/disable audio watermarking.
    #[serde(default)]
    pub watermark_enabled: bool,

    /// Enable streaming output.
    #[serde(default)]
    pub stream: bool,

    /// Speech rate (0.5-2.0, default 1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f32>,

    /// Audio volume (0-10, default 1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<f32>,

    /// Stream encoding format (base64/hex).
    #[serde(default)]
    pub encode_format: StreamEncoding,

    /// Output format (wav/pcm).
    #[serde(default)]
    pub response_format: AudioFormat,
}

/// Response containing generated speech audio.
#[derive(Debug, Clone)]
pub struct SpeechResponse {
    /// The generated audio data.
    pub audio: Bytes,
    /// The content type (audio/wav or audio/pcm).
    pub content_type: String,
}

impl SpeechResponse {
    /// Create a new speech response.
    #[must_use]
    pub fn new(audio: Bytes, content_type: String) -> Self {
        Self {
            audio,
            content_type,
        }
    }

    /// Save the audio to a file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written.
    pub async fn save<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<()> {
        tokio::fs::write(path, &self.audio).await
    }

    /// Get the audio data as bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.audio
    }
}

/// Streaming chunk response for TTS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechStreamChunk {
    /// The ID of the streaming response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Unix timestamp of creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<u64>,
    /// Object type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    /// Model used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Audio data chunk (base64 or hex encoded).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// Whether this is the final chunk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done: Option<bool>,
}
