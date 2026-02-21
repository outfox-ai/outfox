//! Web reader API implementation.

use crate::Client;
use crate::error::{ErrorResponse, Result, ZhipuError};
use crate::spec::tools::{ReturnFormat, WebReaderRequest, WebReaderResponse};

/// Web reader API.
pub struct WebReader<'c> {
    client: &'c Client,
}

impl<'c> WebReader<'c> {
    /// Create a new WebReader API.
    pub(crate) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Read and parse a web page.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn read(&self, request: WebReaderRequest) -> Result<WebReaderResponse> {
        let config = self.client.config();
        let url = config.url("/reader");
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

    /// Simple helper to read a URL and get markdown content.
    pub async fn read_url(&self, url: &str) -> Result<String> {
        let request = WebReaderRequest {
            url: url.to_string(),
            timeout: None,
            no_cache: None,
            return_format: Some(ReturnFormat::Markdown),
            retain_images: Some(true),
            no_gfm: None,
            keep_img_data_url: None,
            with_images_summary: None,
            with_links_summary: None,
            request_id: None,
        };
        let response = self.read(request).await?;
        Ok(response.reader_result.content)
    }

    /// Read URL and return plain text.
    pub async fn read_url_text(&self, url: &str) -> Result<String> {
        let request = WebReaderRequest {
            url: url.to_string(),
            timeout: None,
            no_cache: None,
            return_format: Some(ReturnFormat::Text),
            retain_images: Some(false),
            no_gfm: None,
            keep_img_data_url: None,
            with_images_summary: None,
            with_links_summary: None,
            request_id: None,
        };
        let response = self.read(request).await?;
        Ok(response.reader_result.content)
    }

    /// Read URL with custom timeout.
    pub async fn read_url_with_timeout(&self, url: &str, timeout: u32) -> Result<String> {
        let request = WebReaderRequest {
            url: url.to_string(),
            timeout: Some(timeout),
            no_cache: None,
            return_format: Some(ReturnFormat::Markdown),
            retain_images: Some(true),
            no_gfm: None,
            keep_img_data_url: None,
            with_images_summary: None,
            with_links_summary: None,
            request_id: None,
        };
        let response = self.read(request).await?;
        Ok(response.reader_result.content)
    }
}
