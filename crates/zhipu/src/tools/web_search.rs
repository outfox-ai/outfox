//! Web search API implementation.

use crate::Client;
use crate::error::{ErrorResponse, Result, ZhipuError};
use crate::spec::tools::{SearchEngine, WebSearchRequest, WebSearchResponse};

/// Web search API.
pub struct WebSearch<'c> {
    client: &'c Client,
}

impl<'c> WebSearch<'c> {
    /// Create a new WebSearch API.
    pub(crate) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Perform a web search.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn search(&self, request: WebSearchRequest) -> Result<WebSearchResponse> {
        let config = self.client.config();
        let url = config.url("/web_search");
        let headers = config.headers()?;

        let response = self
            .client
            .http_client()
            .post(&url)
            .headers(headers)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error: ErrorResponse = response.json().await?;
            return Err(ZhipuError::ApiError(error.error));
        }

        let body = response.json().await?;
        Ok(body)
    }

    /// Simple search with standard engine.
    pub async fn search_simple(&self, query: &str) -> Result<WebSearchResponse> {
        let request = WebSearchRequest {
            search_query: query.to_string(),
            search_engine: SearchEngine::Standard,
            search_intent: false,
            count: Some(10),
            search_domain_filter: None,
            search_recency_filter: None,
            content_size: None,
            request_id: None,
            user_id: None,
        };
        self.search(request).await
    }

    /// Search with professional engine.
    pub async fn search_pro(&self, query: &str) -> Result<WebSearchResponse> {
        let request = WebSearchRequest {
            search_query: query.to_string(),
            search_engine: SearchEngine::Pro,
            search_intent: true,
            count: Some(10),
            search_domain_filter: None,
            search_recency_filter: None,
            content_size: None,
            request_id: None,
            user_id: None,
        };
        self.search(request).await
    }

    /// Search with result count limit.
    pub async fn search_with_count(&self, query: &str, count: u32) -> Result<WebSearchResponse> {
        let request = WebSearchRequest {
            search_query: query.to_string(),
            search_engine: SearchEngine::Standard,
            search_intent: false,
            count: Some(count),
            search_domain_filter: None,
            search_recency_filter: None,
            content_size: None,
            request_id: None,
            user_id: None,
        };
        self.search(request).await
    }
}
