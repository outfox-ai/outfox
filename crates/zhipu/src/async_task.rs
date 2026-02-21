//! Async task API implementation.

use crate::Client;
use crate::error::{ErrorResponse, Result, ZhipuError};
use crate::spec::async_task::{
    AsyncChatResult, AsyncImageResult, AsyncTaskResponse, AsyncVideoResult, CreateAsyncChatRequest,
    CreateAsyncImageRequest, CreateAsyncVideoRequest,
};

/// Async task API.
pub struct AsyncTask<'c> {
    client: &'c Client,
}

impl<'c> AsyncTask<'c> {
    /// Create a new AsyncTask API.
    pub(crate) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Create an async chat completion task.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn create_chat(&self, request: CreateAsyncChatRequest) -> Result<AsyncTaskResponse> {
        let config = self.client.config();
        let url = config.url("/async/chat/completions");
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

    /// Create an async video generation task.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn create_video(
        &self,
        request: CreateAsyncVideoRequest,
    ) -> Result<AsyncTaskResponse> {
        let config = self.client.config();
        let url = config.url("/async/videos/generations");
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

    /// Create an async image generation task.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn create_image(
        &self,
        request: CreateAsyncImageRequest,
    ) -> Result<AsyncTaskResponse> {
        let config = self.client.config();
        let url = config.url("/async/images/generations");
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

    /// Query the result of an async chat task.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn get_chat_result(&self, task_id: &str) -> Result<AsyncChatResult> {
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

    /// Query the result of an async video task.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn get_video_result(&self, task_id: &str) -> Result<AsyncVideoResult> {
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

    /// Query the result of an async image task.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn get_image_result(&self, task_id: &str) -> Result<AsyncImageResult> {
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
