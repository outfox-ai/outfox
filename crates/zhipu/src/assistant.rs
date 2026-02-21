//! Assistant API implementation.

use futures_util::StreamExt;
use reqwest_eventsource::{Event, EventSource};
use tokio_stream::Stream;

use crate::Client;
use crate::error::{ErrorResponse, Result, ZhipuError};
use crate::spec::assistant::{
    AssistantCompletion, AssistantConversationRequest, QueryAssistantSupportRequest,
    QueryAssistantSupportResponse, QueryConversationUsageRequest, QueryConversationUsageResponse,
};

/// Assistant API.
pub struct Assistant<'c> {
    client: &'c Client,
}

impl<'c> Assistant<'c> {
    /// Create a new Assistant API.
    pub(crate) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Start a conversation with an assistant.
    ///
    /// # Arguments
    ///
    /// * `request` - The conversation request.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn conversation(
        &self,
        request: AssistantConversationRequest,
    ) -> Result<AssistantCompletion> {
        let config = self.client.config();
        let url = config.url("/assistant");
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

    /// Start a conversation with an assistant with streaming.
    ///
    /// # Arguments
    ///
    /// * `request` - The conversation request.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn conversation_stream(
        &self,
        mut request: AssistantConversationRequest,
    ) -> Result<impl Stream<Item = Result<AssistantCompletion>>> {
        request.stream = Some(true);

        let config = self.client.config();
        let url = config.url("/assistant");
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
                    match serde_json::from_str::<AssistantCompletion>(&msg.data) {
                        Ok(chunk) => Some(Ok(chunk)),
                        Err(e) => Some(Err(ZhipuError::Json(e))),
                    }
                }
                Ok(Event::Open) => None,
                Err(e) => Some(Err(ZhipuError::Stream(e.to_string()))),
            }
        }))
    }

    /// Query assistant support.
    ///
    /// # Arguments
    ///
    /// * `request` - The query request.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn query_support(
        &self,
        request: QueryAssistantSupportRequest,
    ) -> Result<QueryAssistantSupportResponse> {
        let config = self.client.config();
        let url = config.url("/assistant/list");
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

    /// Query conversation usage.
    ///
    /// # Arguments
    ///
    /// * `request` - The query request.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn query_conversation_usage(
        &self,
        request: QueryConversationUsageRequest,
    ) -> Result<QueryConversationUsageResponse> {
        let config = self.client.config();
        let url = config.url("/assistant/conversation/list");
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
}
