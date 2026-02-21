//! Web search request and response types.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::ZhipuError;

/// Search engine options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchEngine {
    /// Standard search engine.
    #[serde(rename = "search_std")]
    Standard,
    /// Professional search engine.
    #[serde(rename = "search_pro")]
    Pro,
    /// Sogou professional search.
    #[serde(rename = "search_pro_sogou")]
    ProSogou,
    /// Quark professional search.
    #[serde(rename = "search_pro_quark")]
    ProQuark,
}

impl Default for SearchEngine {
    fn default() -> Self {
        Self::Standard
    }
}

/// Time range filter for search results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SearchRecencyFilter {
    /// Results from the last day.
    OneDay,
    /// Results from the last week.
    OneWeek,
    /// Results from the last month.
    OneMonth,
    /// Results from the last year.
    OneYear,
    /// No time limit.
    NoLimit,
}

impl Default for SearchRecencyFilter {
    fn default() -> Self {
        Self::NoLimit
    }
}

/// Content detail level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContentSize {
    /// Medium detail level.
    Medium,
    /// High detail level.
    High,
}

/// Request for web search.
#[derive(Clone, Default, Debug, Builder, Serialize, Deserialize)]
#[builder(name = "WebSearchRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "ZhipuError"))]
pub struct WebSearchRequest {
    /// Search query (max 70 characters recommended).
    pub search_query: String,

    /// Search engine to use.
    pub search_engine: SearchEngine,

    /// Enable intent recognition.
    #[serde(default)]
    pub search_intent: bool,

    /// Number of results to return (1-50, default: 10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,

    /// Whitelist domains to filter results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_domain_filter: Option<String>,

    /// Time range filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_recency_filter: Option<SearchRecencyFilter>,

    /// Response detail level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_size: Option<ContentSize>,

    /// Request ID for tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,

    /// User identifier (6-128 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// Search intent analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchIntent {
    /// Intent type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent: Option<String>,
    /// Confidence score.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
}

/// A single search result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Result title.
    pub title: String,
    /// Result content/snippet.
    pub content: String,
    /// Result URL.
    pub link: String,
    /// Media source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<String>,
    /// Favicon URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// Reference information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refer: Option<String>,
    /// Publication date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_date: Option<String>,
}

/// Response from web search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSearchResponse {
    /// Unique identifier.
    pub id: String,
    /// Unix timestamp.
    pub created: u64,
    /// Request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Intent analysis results.
    #[serde(default)]
    pub search_intent: Vec<SearchIntent>,
    /// Search results.
    #[serde(default)]
    pub search_result: Vec<SearchResult>,
}
