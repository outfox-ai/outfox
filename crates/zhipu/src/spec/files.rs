//! Files API request and response types.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Purpose of the file upload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FilePurpose {
    /// For fine-tuning.
    FineTune,
    /// For retrieval.
    Retrieval,
    /// For batch processing.
    Batch,
    /// For voice cloning input.
    VoiceCloneInput,
}

/// Upload detail for file upload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadDetail {
    /// URL of the file.
    pub url: String,
    /// Optional filename.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

/// File upload request.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(name = "CreateFileRequestArgs", setter(into, strip_option))]
pub struct CreateFileRequest {
    /// Purpose of the file.
    pub purpose: FilePurpose,
    /// Knowledge ID (optional).
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_id: Option<String>,
    /// Sentence size for chunking (optional).
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentence_size: Option<i32>,
}

/// File object response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileObject {
    /// Unique identifier for the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Size of the file in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<i64>,
    /// Timestamp when the file was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// Name of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// Object type identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    /// Purpose of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    /// Current status of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Additional details about the file status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
}

/// List of file objects response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFilesResponse {
    /// Object type identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    /// List of file objects.
    pub data: Vec<FileObject>,
    /// Whether there are more files available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// File deletion response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDeleted {
    /// ID of the deleted file.
    pub id: String,
    /// Object type.
    pub object: String,
    /// Whether the file was deleted.
    pub deleted: bool,
}

/// List files query parameters.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(name = "ListFilesQueryArgs", setter(into, strip_option), default)]
pub struct ListFilesQuery {
    /// Filter by purpose.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    /// Limit number of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Order of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}
