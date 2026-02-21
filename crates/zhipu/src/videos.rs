//! Videos API implementation.

use crate::Client;
use crate::error::{ErrorResponse, Result, ZhipuError};
use crate::spec::videos::{GenerateVideoRequest, VideoObject};

/// Videos API.
pub struct Videos<'c> {
    client: &'c Client,
}

impl<'c> Videos<'c> {
    /// Create a new Videos API.
    pub(crate) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Generate a video.
    ///
    /// # Arguments
    ///
    /// * `request` - The video generation request.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn generate(&self, request: GenerateVideoRequest) -> Result<VideoObject> {
        let config = self.client.config();
        let url = config.url("/videos/generations");
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

    /// Retrieve video generation result.
    ///
    /// # Arguments
    ///
    /// * `task_id` - The ID of the video generation task.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn retrieve(&self, task_id: &str) -> Result<VideoObject> {
        let config = self.client.config();
        let url = config.url(&format!("/async-result/{}", task_id));
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
}
