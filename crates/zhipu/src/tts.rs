//! Text-to-speech API implementation.

use bytes::Bytes;
use futures_util::StreamExt;
use reqwest_eventsource::{Event, EventSource};
use tokio_stream::Stream;

use crate::Client;
use crate::error::{ErrorResponse, Result, ZhipuError};
use crate::spec::tts::{CreateSpeechRequest, SpeechResponse, SpeechStreamChunk, Voice};

/// Text-to-speech API.
pub struct Tts<'c> {
    client: &'c Client,
}

impl<'c> Tts<'c> {
    /// Create a new TTS API.
    pub(crate) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Create speech from text.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn create(&self, request: CreateSpeechRequest) -> Result<SpeechResponse> {
        let config = self.client.config();
        let url = config.url("/audio/speech");
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
            // Try to parse error response
            let error_text = response.text().await?;
            if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&error_text) {
                return Err(ZhipuError::ApiError(error_response.error));
            }
            return Err(ZhipuError::InvalidArgument(format!(
                "API error: {}",
                error_text
            )));
        }

        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("audio/wav")
            .to_string();

        let bytes = response.bytes().await?;
        Ok(SpeechResponse::new(bytes, content_type))
    }

    /// Create speech from text with streaming.
    ///
    /// Returns a stream of audio chunks.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn create_stream(
        &self,
        mut request: CreateSpeechRequest,
    ) -> Result<impl Stream<Item = Result<SpeechStreamChunk>>> {
        request.stream = true;

        let config = self.client.config();
        let url = config.url("/audio/speech");
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
                    match serde_json::from_str::<SpeechStreamChunk>(&msg.data) {
                        Ok(chunk) => Some(Ok(chunk)),
                        Err(e) => Some(Err(ZhipuError::Json(e))),
                    }
                }
                Ok(Event::Open) => None,
                Err(e) => Some(Err(ZhipuError::Stream(e.to_string()))),
            }
        }))
    }

    /// Simple helper to synthesize speech from text.
    ///
    /// Uses default voice (tongtong) and WAV format.
    pub async fn synthesize(&self, text: &str) -> Result<SpeechResponse> {
        let request = CreateSpeechRequest {
            input: text.to_string(),
            ..Default::default()
        };
        self.create(request).await
    }

    /// Synthesize speech with a specific voice.
    pub async fn synthesize_with_voice(&self, text: &str, voice: Voice) -> Result<SpeechResponse> {
        let request = CreateSpeechRequest {
            input: text.to_string(),
            voice,
            ..Default::default()
        };
        self.create(request).await
    }

    /// Synthesize speech and save to a file.
    ///
    /// # Errors
    ///
    /// Returns an error if synthesis fails or the file cannot be written.
    pub async fn synthesize_to_file<P: AsRef<std::path::Path>>(
        &self,
        text: &str,
        path: P,
    ) -> Result<()> {
        let response = self.synthesize(text).await?;
        response
            .save(path)
            .await
            .map_err(|e| ZhipuError::FileError(e.to_string()))
    }

    /// Decode streaming audio chunk from base64/hex.
    ///
    /// Returns the decoded audio bytes.
    pub fn decode_chunk(chunk: &SpeechStreamChunk, is_hex: bool) -> Result<Bytes> {
        let data = chunk
            .data
            .as_ref()
            .ok_or_else(|| ZhipuError::InvalidArgument("missing audio data".to_string()))?;

        if is_hex {
            hex::decode(data)
                .map(Bytes::from)
                .map_err(|e| ZhipuError::InvalidArgument(format!("invalid hex: {}", e)))
        } else {
            use base64::Engine;
            base64::engine::general_purpose::STANDARD
                .decode(data)
                .map(Bytes::from)
                .map_err(|e| ZhipuError::InvalidArgument(format!("invalid base64: {}", e)))
        }
    }
}
