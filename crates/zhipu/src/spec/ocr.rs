//! OCR API request and response types.

use serde::{Deserialize, Serialize};

/// OCR tool type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OcrToolType {
    /// Handwriting recognition.
    HandWrite,
}

/// OCR request (used with multipart form upload).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrRequest {
    /// Tool type.
    pub tool_type: OcrToolType,
    /// Language type (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_type: Option<String>,
    /// Whether to include probability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probability: Option<bool>,
}

/// OCR word result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrWord {
    /// Word text.
    pub word: String,
    /// Bounding box coordinates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<OcrLocation>,
    /// Confidence score.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probability: Option<f64>,
}

/// OCR location (bounding box).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrLocation {
    /// Top-left X coordinate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<i32>,
    /// Top-left Y coordinate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
    /// Width.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    /// Height.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
}

/// OCR response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrResponse {
    /// Log ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_id: Option<String>,
    /// Word results.
    #[serde(default)]
    pub words_result: Vec<OcrWord>,
    /// Number of words detected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub words_result_num: Option<i32>,
}
