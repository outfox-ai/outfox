//! Chat completions API implementation.

use futures_util::StreamExt;
use reqwest_eventsource::{Event, EventSource};
use tokio_stream::Stream;

use crate::Client;
use crate::error::{ErrorResponse, Result, ZhipuError};
use crate::spec::chat::{
    ChatCompletionChunk, CreateChatCompletionRequest, CreateChatCompletionResponse,
};

/// Chat completions API.
pub struct Chat<'c> {
    client: &'c Client,
}

impl<'c> Chat<'c> {
    /// Create a new Chat API.
    pub(crate) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Create a chat completion.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn create(
        &self,
        request: CreateChatCompletionRequest,
    ) -> Result<CreateChatCompletionResponse> {
        let config = self.client.config();
        let url = config.url("/chat/completions");
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

    /// Create a chat completion with streaming.
    ///
    /// Returns a stream of completion chunks.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn create_stream(
        &self,
        mut request: CreateChatCompletionRequest,
    ) -> Result<impl Stream<Item = Result<ChatCompletionChunk>>> {
        request.stream = Some(true);

        let config = self.client.config();
        let url = config.url("/chat/completions");
        let headers = config.headers()?;

        let request_builder = self
            .client
            .http_client()
            .post(&url)
            .headers(headers)
            .json(&request);

        let event_source =
            EventSource::new(request_builder).map_err(|e| ZhipuError::Stream(e.to_string()))?;

        Ok(event_source.filter_map(|event| async move {
            match event {
                Ok(Event::Message(msg)) => {
                    if msg.data == "[DONE]" {
                        return None;
                    }
                    match serde_json::from_str::<ChatCompletionChunk>(&msg.data) {
                        Ok(chunk) => Some(Ok(chunk)),
                        Err(e) => Some(Err(ZhipuError::Json(e))),
                    }
                }
                Ok(Event::Open) => None,
                Err(e) => Some(Err(ZhipuError::Stream(e.to_string()))),
            }
        }))
    }
}
