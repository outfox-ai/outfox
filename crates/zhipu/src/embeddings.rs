//! Embeddings API implementation.

use crate::Client;
use crate::error::{ErrorResponse, Result, ZhipuError};
use crate::spec::embeddings::{CreateEmbeddingsRequest, CreateEmbeddingsResponse, EmbeddingInput};

/// Embeddings API.
pub struct Embeddings<'c> {
    client: &'c Client,
}

impl<'c> Embeddings<'c> {
    /// Create a new Embeddings API.
    pub(crate) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Create embeddings for the given input.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn create(
        &self,
        request: CreateEmbeddingsRequest,
    ) -> Result<CreateEmbeddingsResponse> {
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
            return Err(ZhipuError::ApiError(error.error));
        }

        let body = response.json().await?;
        Ok(body)
    }

    /// Create embeddings for a single text.
    ///
    /// Convenience method for embedding a single string.
    pub async fn embed_text(&self, model: &str, text: &str) -> Result<Vec<f32>> {
        let request = CreateEmbeddingsRequest {
            model: model.to_string(),
            input: EmbeddingInput::Single(text.to_string()),
        };

        let response = self.create(request).await?;

        response
            .data
            .into_iter()
            .next()
            .map(|e| e.embedding)
            .ok_or_else(|| ZhipuError::InvalidArgument("no embedding returned".to_string()))
    }

    /// Create embeddings for multiple texts.
    ///
    /// Convenience method for embedding multiple strings.
    pub async fn embed_texts(&self, model: &str, texts: Vec<String>) -> Result<Vec<Vec<f32>>> {
        let request = CreateEmbeddingsRequest {
            model: model.to_string(),
            input: EmbeddingInput::Multiple(texts),
        };

        let response = self.create(request).await?;

        Ok(response.data.into_iter().map(|e| e.embedding).collect())
    }
}
