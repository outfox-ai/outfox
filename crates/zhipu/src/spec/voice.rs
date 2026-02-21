//! Voice API request and response types.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::ZhipuError;

/// Voice type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum VoiceType {
    /// Official system voice.
    Official,
    /// User's private cloned voice.
    Private,
}

/// Voice information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceInfo {
    /// Voice identifier.
    pub voice: String,
    /// User-assigned voice name.
    pub voice_name: String,
    /// Voice type (official or private).
    pub voice_type: VoiceType,
    /// Sample audio download URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    /// Creation timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
}

/// Request to clone a voice.
#[derive(Clone, Default, Debug, Builder, Serialize, Deserialize)]
#[builder(name = "VoiceCloneRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "ZhipuError"))]
pub struct VoiceCloneRequest {
    /// Model to use (fixed: "glm-tts-clone").
    #[builder(default = "\"glm-tts-clone\".to_string()")]
    pub model: String,

    /// Unique identifier for the cloned voice.
    pub voice_name: String,

    /// Target text for audio generation.
    pub input: String,

    /// Audio file ID (uploaded via file API).
    pub file_id: String,

    /// Text content of the sample audio (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Request ID for tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// Response from voice cloning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceCloneResponse {
    /// Generated voice identifier.
    pub voice: String,
    /// Output audio file ID.
    pub file_id: String,
    /// File purpose (always "voice-clone-output").
    pub file_purpose: String,
    /// Request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// Query parameters for voice list.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VoiceListQuery {
    /// Filter by voice name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "voiceName")]
    pub voice_name: Option<String>,
    /// Filter by voice type.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "voiceType")]
    pub voice_type: Option<VoiceType>,
}

/// Response from voice list query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceListResponse {
    /// List of voices.
    pub voice_list: Vec<VoiceInfo>,
}

/// Request to delete a voice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceDeleteRequest {
    /// Voice identifier to delete.
    pub voice: String,
    /// Request ID for tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// Response from voice deletion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceDeleteResponse {
    /// Deleted voice identifier.
    pub voice: String,
    /// Deletion timestamp.
    pub update_time: String,
}
