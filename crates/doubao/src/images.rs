//! Images generation API implementation.

use futures_util::StreamExt;
use reqwest_eventsource::{Event, EventSource};
use tokio_stream::Stream;

use crate::Client;
use crate::error::{ApiError, DoubaoError, Result};
use crate::spec::images::{
    GenerateImagesRequest, GenerateImagesResponse, GenerateImagesStreamResponse,
};

/// API error response wrapper.
#[derive(Debug, serde::Deserialize)]
struct ErrorResponse {
    error: ApiError,
}

/// Images generation API.
pub struct Images<'c> {
    client: &'c Client,
}

impl<'c> Images<'c> {
    /// Create a new Images API.
    pub(crate) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Generate images from a text prompt.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn generate(&self, request: GenerateImagesRequest) -> Result<GenerateImagesResponse> {
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
            return Err(DoubaoError::ApiError(error.error));
        }

        let body = response.json().await?;
        Ok(body)
    }

    /// Generate images with streaming response.
    ///
    /// Returns a stream of image generation events.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn generate_stream(
        &self,
        request: GenerateImagesRequest,
    ) -> Result<impl Stream<Item = Result<GenerateImagesStreamResponse>>> {
        let config = self.client.config();
        let url = config.url("/images/generations");
        let headers = config.headers()?;

        // Add stream: true to the request
        let mut body = serde_json::to_value(&request)?;
        if let Some(obj) = body.as_object_mut() {
            obj.insert("stream".to_string(), serde_json::Value::Bool(true));
        }

        let request_builder = self
            .client
            .http_client()
            .post(&url)
            .headers(headers)
            .json(&body);

        let event_source =
            EventSource::new(request_builder).map_err(|e| DoubaoError::Stream(e.to_string()))?;

        Ok(event_source.filter_map(|event| async move {
            match event {
                Ok(Event::Message(msg)) => {
                    if msg.data == "[DONE]" {
                        return None;
                    }
                    match serde_json::from_str::<GenerateImagesStreamResponse>(&msg.data) {
                        Ok(chunk) => Some(Ok(chunk)),
                        Err(e) => Some(Err(DoubaoError::Json(e))),
                    }
                }
                Ok(Event::Open) => None,
                Err(e) => Some(Err(DoubaoError::Stream(e.to_string()))),
            }
        }))
    }
}
