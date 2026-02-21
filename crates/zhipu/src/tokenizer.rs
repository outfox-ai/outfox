//! Text tokenizer API implementation.

use crate::Client;
use crate::error::{ErrorResponse, Result, ZhipuError};
use crate::spec::chat::ChatMessage;
use crate::spec::tokenizer::{TokenizerRequest, TokenizerResponse};

/// Text tokenizer API.
pub struct Tokenizer<'c> {
    client: &'c Client,
}

impl<'c> Tokenizer<'c> {
    /// Create a new Tokenizer API.
    pub(crate) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Count tokens for messages.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn tokenize(&self, request: TokenizerRequest) -> Result<TokenizerResponse> {
        let config = self.client.config();
        let url = config.url("/tokenizer");
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

    /// Simple helper to count tokens for a single text message.
    pub async fn count_tokens(&self, model: &str, text: &str) -> Result<u32> {
        let request = TokenizerRequest {
            model: model.to_string(),
            messages: vec![ChatMessage::user(text)],
            tools: None,
            request_id: None,
            user_id: None,
        };
        let response = self.tokenize(request).await?;
        Ok(response.usage.total_tokens)
    }

    /// Count tokens for a conversation.
    pub async fn count_conversation_tokens(
        &self,
        model: &str,
        messages: Vec<ChatMessage>,
    ) -> Result<u32> {
        let request = TokenizerRequest {
            model: model.to_string(),
            messages,
            tools: None,
            request_id: None,
            user_id: None,
        };
        let response = self.tokenize(request).await?;
        Ok(response.usage.total_tokens)
    }
}
