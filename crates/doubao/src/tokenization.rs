//! Tokenization API implementation.

use crate::Client;
use crate::error::{ApiError, DoubaoError, Result};
use crate::spec::tokenization::{CreateTokenizationRequest, CreateTokenizationResponse};

/// API error response wrapper.
#[derive(Debug, serde::Deserialize)]
struct ErrorResponse {
    error: ApiError,
}

/// Tokenization API.
pub struct Tokenization<'c> {
    client: &'c Client,
}

impl<'c> Tokenization<'c> {
    /// Create a new Tokenization API.
    pub(crate) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Tokenize text.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn create(
        &self,
        request: CreateTokenizationRequest,
    ) -> Result<CreateTokenizationResponse> {
        let config = self.client.config();
        let url = config.url("/tokenization");
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
