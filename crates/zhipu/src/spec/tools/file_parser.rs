//! File parsing request and response types.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::ZhipuError;

/// Parser tool type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParserToolType {
    /// Lite parser (PDF, DOCX, DOC, XLS, XLSX, PPT, PPTX, PNG, JPG, JPEG, CSV, TXT, MD).
    Lite,
    /// Expert parser (PDF only, high quality).
    Expert,
    /// Prime parser (20+ formats).
    Prime,
}

impl Default for ParserToolType {
    fn default() -> Self {
        Self::Lite
    }
}

/// Parse result format type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParseResultFormat {
    /// Return parsed text content.
    Text,
    /// Return download link for results.
    DownloadLink,
}

/// Parsing task status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParseStatus {
    /// Task is being processed.
    Processing,
    /// Task completed successfully.
    Succeeded,
    /// Task failed.
    Failed,
}

/// Request to create a file parsing task.
/// Note: This is a multipart form request - the actual file is sent separately.
#[derive(Clone, Default, Debug, Builder, Serialize, Deserialize)]
#[builder(name = "FileParseRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "ZhipuError"))]
pub struct FileParseRequest {
    /// Parser tool to use.
    pub tool_type: ParserToolType,

    /// File type hint (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
}

/// Response from file parsing task creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileParseResponse {
    /// Whether the task was created successfully.
    pub success: bool,
    /// Status message.
    pub message: String,
    /// Task ID for querying results.
    pub task_id: String,
}

/// Response from parsing result query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileParseResultResponse {
    /// Processing status.
    pub status: ParseStatus,
    /// Status description.
    pub message: String,
    /// Task ID.
    pub task_id: String,
    /// Parsed text content (when format_type=text).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Download link for results (when format_type=download_link).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsing_result_url: Option<String>,
}
