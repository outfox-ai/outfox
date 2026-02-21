//! Text reranking API implementation.

use crate::Client;
use crate::error::{ErrorResponse, Result, ZhipuError};
use crate::spec::reranking::{RerankRequest, RerankResponse};

/// Text reranking API.
pub struct Reranking<'c> {
    client: &'c Client,
}

impl<'c> Reranking<'c> {
    /// Create a new Reranking API.
    pub(crate) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Rerank documents by relevance to a query.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn rerank(&self, request: RerankRequest) -> Result<RerankResponse> {
        let config = self.client.config();
        let url = config.url("/rerank");
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

    /// Simple helper to rerank documents.
    ///
    /// Returns documents sorted by relevance.
    pub async fn rerank_simple(
        &self,
        query: &str,
        documents: Vec<String>,
    ) -> Result<RerankResponse> {
        let request = RerankRequest {
            model: "rerank".to_string(),
            query: query.to_string(),
            documents,
            top_n: None,
            return_documents: Some(true),
            return_raw_scores: None,
            request_id: None,
            user_id: None,
        };
        self.rerank(request).await
    }

    /// Rerank and return top N results.
    pub async fn rerank_top_n(
        &self,
        query: &str,
        documents: Vec<String>,
        top_n: u32,
    ) -> Result<RerankResponse> {
        let request = RerankRequest {
            model: "rerank".to_string(),
            query: query.to_string(),
            documents,
            top_n: Some(top_n),
            return_documents: Some(true),
            return_raw_scores: None,
            request_id: None,
            user_id: None,
        };
        self.rerank(request).await
    }
}
