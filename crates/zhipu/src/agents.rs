//! Agents API implementation.

use futures_util::StreamExt;
use reqwest_eventsource::{Event, EventSource};
use tokio_stream::Stream;

use crate::Client;
use crate::error::{ErrorResponse, Result, ZhipuError};
use crate::spec::agents::{
    AgentAsyncResultRequest, AgentCompletion, AgentCompletionChunk, InvokeAgentRequest,
};

/// Agents API.
pub struct Agents<'c> {
    client: &'c Client,
}

impl<'c> Agents<'c> {
    /// Create a new Agents API.
    pub(crate) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Invoke an agent.
    ///
    /// # Arguments
    ///
    /// * `request` - The agent invocation request.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn invoke(&self, request: InvokeAgentRequest) -> Result<AgentCompletion> {
        let config = self.client.config();
        let url = config.url("/v1/agents");
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

    /// Invoke an agent with streaming.
    ///
    /// # Arguments
    ///
    /// * `request` - The agent invocation request.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn invoke_stream(
        &self,
        mut request: InvokeAgentRequest,
    ) -> Result<impl Stream<Item = Result<AgentCompletionChunk>>> {
        request.stream = Some(true);

        let config = self.client.config();
        let url = config.url("/v1/agents");
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
                    match serde_json::from_str::<AgentCompletionChunk>(&msg.data) {
                        Ok(chunk) => Some(Ok(chunk)),
                        Err(e) => Some(Err(ZhipuError::Json(e))),
                    }
                }
                Ok(Event::Open) => None,
                Err(e) => Some(Err(ZhipuError::Stream(e.to_string()))),
            }
        }))
    }

    /// Get async agent result.
    ///
    /// # Arguments
    ///
    /// * `request` - The async result request.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn async_result(&self, request: AgentAsyncResultRequest) -> Result<AgentCompletion> {
        let config = self.client.config();
        let url = config.url("/v1/agents/async-result");
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
