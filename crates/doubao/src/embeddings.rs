//! Embeddings API implementation.

use crate::Client;
use crate::error::{ApiError, DoubaoError, Result};
use crate::spec::embeddings::{
    CreateEmbeddingRequest, CreateEmbeddingResponse, CreateMultimodalEmbeddingRequest,
    CreateMultimodalEmbeddingResponse,
};

/// API error response wrapper.
#[derive(Debug, serde::Deserialize)]
struct ErrorResponse {
    error: ApiError,
}

/// Embeddings API.
pub struct Embeddings<'c> {
    client: &'c Client,
}

impl<'c> Embeddings<'c> {
    /// Create a new Embeddings API.
    pub(crate) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Create embeddings for text inputs.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn create(&self, request: CreateEmbeddingRequest) -> Result<CreateEmbeddingResponse> {
        let config = self.client.config();
        let url = config.url("/embeddings");
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
            return Err(DoubaoError::ApiError(error.error));
        }

        let body = response.json().await?;
        Ok(body)
    }

    /// Create multimodal embeddings for text, image, and video inputs.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn create_multimodal(
        &self,
        request: CreateMultimodalEmbeddingRequest,
    ) -> Result<CreateMultimodalEmbeddingResponse> {
        let config = self.client.config();
        let url = config.url("/embeddings/multimodal");
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
            return Err(DoubaoError::ApiError(error.error));
        }

        let body = response.json().await?;
        Ok(body)
    }
}
