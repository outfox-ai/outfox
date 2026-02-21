//! Content moderation API implementation.

use crate::Client;
use crate::error::{ErrorResponse, Result, ZhipuError};
use crate::spec::tools::{
    ModerationInput, ModerationInputType, ModerationRequest, ModerationResponse,
};

/// Content moderation API.
pub struct Moderation<'c> {
    client: &'c Client,
}

impl<'c> Moderation<'c> {
    /// Create a new Moderation API.
    pub(crate) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Moderate content for safety.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn moderate(&self, request: ModerationRequest) -> Result<ModerationResponse> {
        let config = self.client.config();
        let url = config.url("/moderations");
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

    /// Moderate text content.
    pub async fn moderate_text(&self, text: &str) -> Result<ModerationResponse> {
        let request = ModerationRequest {
            model: "moderation".to_string(),
            input: ModerationInputType::Text(text.to_string()),
            request_id: None,
        };
        self.moderate(request).await
    }

    /// Moderate image content.
    pub async fn moderate_image(&self, image_url: &str) -> Result<ModerationResponse> {
        let request = ModerationRequest {
            model: "moderation".to_string(),
            input: ModerationInputType::Single(ModerationInput::image(image_url)),
            request_id: None,
        };
        self.moderate(request).await
    }

    /// Moderate video content.
    pub async fn moderate_video(&self, video_url: &str) -> Result<ModerationResponse> {
        let request = ModerationRequest {
            model: "moderation".to_string(),
            input: ModerationInputType::Single(ModerationInput::video(video_url)),
            request_id: None,
        };
        self.moderate(request).await
    }

    /// Moderate audio content.
    pub async fn moderate_audio(&self, audio_url: &str) -> Result<ModerationResponse> {
        let request = ModerationRequest {
            model: "moderation".to_string(),
            input: ModerationInputType::Single(ModerationInput::audio(audio_url)),
            request_id: None,
        };
        self.moderate(request).await
    }

    /// Moderate multiple content items.
    pub async fn moderate_multiple(
        &self,
        inputs: Vec<ModerationInput>,
    ) -> Result<ModerationResponse> {
        let request = ModerationRequest {
            model: "moderation".to_string(),
            input: ModerationInputType::Multiple(inputs),
            request_id: None,
        };
        self.moderate(request).await
    }
}
