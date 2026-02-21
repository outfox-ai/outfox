//! Batch API implementation.

use crate::Client;
use crate::error::{ErrorResponse, Result, ZhipuError};
use crate::spec::batch::{Batch, CreateBatchRequest, ListBatchesQuery, ListBatchesResponse};

/// Batch API.
pub struct Batches<'c> {
    client: &'c Client,
}

impl<'c> Batches<'c> {
    /// Create a new Batch API.
    pub(crate) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Create a batch.
    ///
    /// # Arguments
    ///
    /// * `request` - The batch creation request.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn create(&self, request: CreateBatchRequest) -> Result<Batch> {
        let config = self.client.config();
        let url = config.url("/batches");
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

    /// Retrieve a batch.
    ///
    /// # Arguments
    ///
    /// * `batch_id` - The ID of the batch.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn retrieve(&self, batch_id: &str) -> Result<Batch> {
        let config = self.client.config();
        let url = config.url(&format!("/batches/{}", batch_id));
        let headers = config.headers()?;

        let response = self
            .client
            .http_client()
            .get(&url)
            .headers(headers)
            .send()
            .await?;

        if !response.status().is_success() {
            let error: ErrorResponse = response.json().await?;
            return Err(ZhipuError::ApiError(error.error));
        }

        let body = response.json().await?;
        Ok(body)
    }

    /// List batches.
    ///
    /// # Arguments
    ///
    /// * `query` - Optional query parameters.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn list(&self, query: Option<ListBatchesQuery>) -> Result<ListBatchesResponse> {
        let config = self.client.config();
        let mut url = config.url("/batches");
        let headers = config.headers()?;

        if let Some(q) = &query {
            let mut params = vec![];
            if let Some(limit) = q.limit {
                params.push(format!("limit={}", limit));
            }
            if let Some(after) = &q.after {
                params.push(format!("after={}", after));
            }
            if !params.is_empty() {
                url = format!("{}?{}", url, params.join("&"));
            }
        }

        let response = self
            .client
            .http_client()
            .get(&url)
            .headers(headers)
            .send()
            .await?;

        if !response.status().is_success() {
            let error: ErrorResponse = response.json().await?;
            return Err(ZhipuError::ApiError(error.error));
        }

        let body = response.json().await?;
        Ok(body)
    }

    /// Cancel a batch.
    ///
    /// # Arguments
    ///
    /// * `batch_id` - The ID of the batch to cancel.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn cancel(&self, batch_id: &str) -> Result<Batch> {
        let config = self.client.config();
        let url = config.url(&format!("/batches/{}/cancel", batch_id));
        let headers = config.headers()?;

        let response = self
            .client
            .http_client()
            .post(&url)
            .headers(headers)
            .send()
            .await?;

        if !response.status().is_success() {
            let error: ErrorResponse = response.json().await?;
            return Err(ZhipuError::ApiError(error.error));
        }

        let body = response.json().await?;
        Ok(body)
    }
}
