//! Images API implementation.

use crate::Client;
use crate::error::{ErrorResponse, Result, ZhipuError};
use crate::spec::images::{CreateImageRequest, CreateImageResponse, ImageBytes};

/// Images API.
pub struct Images<'c> {
    client: &'c Client,
}

impl<'c> Images<'c> {
    /// Create a new Images API.
    pub(crate) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Create images from a prompt.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn create(&self, request: CreateImageRequest) -> Result<CreateImageResponse> {
        let config = self.client.config();
        let url = config.url("/images/generations");
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

    /// Generate an image and return as bytes.
    ///
    /// Downloads the generated image from the URL.
    pub async fn generate(&self, model: &str, prompt: &str) -> Result<ImageBytes> {
        let request = CreateImageRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
            ..Default::default()
        };

        let response = self.create(request).await?;

        let url = response
            .first_url()
            .ok_or_else(|| ZhipuError::InvalidArgument("no image URL returned".to_string()))?;

        let image_response = self.client.http_client().get(url).send().await?;

        if !image_response.status().is_success() {
            return Err(ZhipuError::InvalidArgument(format!(
                "failed to download image: {}",
                image_response.status()
            )));
        }

        let bytes = image_response.bytes().await?;
        Ok(ImageBytes { bytes })
    }

    /// Generate an image with specific size.
    pub async fn generate_with_size(
        &self,
        model: &str,
        prompt: &str,
        size: &str,
    ) -> Result<ImageBytes> {
        let request = CreateImageRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
            size: Some(size.to_string()),
            ..Default::default()
        };

        let response = self.create(request).await?;

        let url = response
            .first_url()
            .ok_or_else(|| ZhipuError::InvalidArgument("no image URL returned".to_string()))?;

        let image_response = self.client.http_client().get(url).send().await?;

        if !image_response.status().is_success() {
            return Err(ZhipuError::InvalidArgument(format!(
                "failed to download image: {}",
                image_response.status()
            )));
        }

        let bytes = image_response.bytes().await?;
        Ok(ImageBytes { bytes })
    }
}
