//! Web reader request and response types.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::ZhipuError;

/// Return format options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReturnFormat {
    /// Markdown format.
    Markdown,
    /// Plain text format.
    Text,
}

impl Default for ReturnFormat {
    fn default() -> Self {
        Self::Markdown
    }
}

/// Request to read/parse a web page.
#[derive(Clone, Default, Debug, Builder, Serialize, Deserialize)]
#[builder(name = "WebReaderRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "ZhipuError"))]
pub struct WebReaderRequest {
    /// URL to fetch and parse.
    pub url: String,

    /// Request timeout in seconds (default: 20).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,

    /// Disable caching.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_cache: Option<bool>,

    /// Response format (markdown or text).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_format: Option<ReturnFormat>,

    /// Keep images in response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_images: Option<bool>,

    /// Disable GitHub Flavored Markdown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_gfm: Option<bool>,

    /// Preserve image data URLs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_img_data_url: Option<bool>,

    /// Include image summaries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_images_summary: Option<bool>,

    /// Include link summaries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_links_summary: Option<bool>,

    /// Request ID for tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// Parsed page content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReaderResult {
    /// Main page content.
    pub content: String,
    /// Page title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Page description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Original URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// External resources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<serde_json::Value>,
    /// Page metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Response from web reader.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebReaderResponse {
    /// Unique identifier.
    pub id: String,
    /// Unix timestamp.
    pub created: u64,
    /// Request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Model used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Parsed page content.
    pub reader_result: ReaderResult,
}
