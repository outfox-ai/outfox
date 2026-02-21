//! Speech-to-text (ASR) API implementation.

use futures_util::StreamExt;
use reqwest::multipart::{Form, Part};
use reqwest_eventsource::{Event, EventSource};
use tokio_stream::Stream;

use crate::Client;
use crate::error::{ErrorResponse, Result, ZhipuError};
use crate::spec::asr::{
    AudioInput, CreateTranscriptionRequest, TranscriptionResponse, TranscriptionStreamChunk,
};

/// Speech-to-text (ASR) API.
pub struct Asr<'c> {
    client: &'c Client,
}

impl<'c> Asr<'c> {
    /// Create a new ASR API.
    pub(crate) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Transcribe audio to text.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn transcribe(
        &self,
        request: CreateTranscriptionRequest,
    ) -> Result<TranscriptionResponse> {
        let config = self.client.config();
        let url = config.url("/audio/transcriptions");
        let headers = config.headers()?;

        let form = self.build_form(&request)?;

        let response = self
            .client
            .http_client()
            .post(&url)
            .headers(headers)
            .multipart(form)
            .send()
            .await?;

        if !response.status().is_success() {
            let error: ErrorResponse = response.json().await?;
            return Err(ZhipuError::ApiError(error.error));
        }

        let body = response.json().await?;
        Ok(body)
    }

    /// Transcribe audio with streaming response.
    ///
    /// Returns a stream of transcription chunks.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn transcribe_stream(
        &self,
        mut request: CreateTranscriptionRequest,
    ) -> Result<impl Stream<Item = Result<TranscriptionStreamChunk>>> {
        request.stream = Some(true);

        let config = self.client.config();
        let url = config.url("/audio/transcriptions");
        let headers = config.headers()?;

        let form = self.build_form(&request)?;

        let request_builder = self
            .client
            .http_client()
            .post(&url)
            .headers(headers)
            .multipart(form);

        let event_source =
            EventSource::new(request_builder).map_err(|e| ZhipuError::Stream(e.to_string()))?;

        Ok(event_source.filter_map(|event| async move {
            match event {
                Ok(Event::Message(msg)) => {
                    if msg.data == "[DONE]" {
                        return None;
                    }
                    match serde_json::from_str::<TranscriptionStreamChunk>(&msg.data) {
                        Ok(chunk) => Some(Ok(chunk)),
                        Err(e) => Some(Err(ZhipuError::Json(e))),
                    }
                }
                Ok(Event::Open) => None,
                Err(e) => Some(Err(ZhipuError::Stream(e.to_string()))),
            }
        }))
    }

    /// Simple helper to transcribe an audio file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or transcription fails.
    pub async fn transcribe_file<P: AsRef<std::path::Path>>(
        &self,
        path: P,
    ) -> Result<TranscriptionResponse> {
        let audio = AudioInput::from_path(path)
            .await
            .map_err(|e| ZhipuError::FileError(e.to_string()))?;

        let request = CreateTranscriptionRequest {
            audio: Some(audio),
            ..Default::default()
        };

        self.transcribe(request).await
    }

    /// Transcribe audio from base64-encoded data.
    pub async fn transcribe_base64(&self, base64_data: &str) -> Result<TranscriptionResponse> {
        let request = CreateTranscriptionRequest {
            audio: Some(AudioInput::from_base64(base64_data)),
            ..Default::default()
        };

        self.transcribe(request).await
    }

    /// Transcribe audio with hotwords for better recognition.
    pub async fn transcribe_with_hotwords<P: AsRef<std::path::Path>>(
        &self,
        path: P,
        hotwords: Vec<String>,
    ) -> Result<TranscriptionResponse> {
        let audio = AudioInput::from_path(path)
            .await
            .map_err(|e| ZhipuError::FileError(e.to_string()))?;

        let request = CreateTranscriptionRequest {
            audio: Some(audio),
            hotwords: Some(hotwords),
            ..Default::default()
        };

        self.transcribe(request).await
    }

    /// Build multipart form from request.
    fn build_form(&self, request: &CreateTranscriptionRequest) -> Result<Form> {
        let mut form = Form::new().text("model", request.model.as_str().to_string());

        // Add audio input
        match &request.audio {
            Some(AudioInput::File { data, filename }) => {
                let part = Part::bytes(data.to_vec())
                    .file_name(filename.clone())
                    .mime_str("audio/wav")
                    .map_err(|e| ZhipuError::InvalidArgument(e.to_string()))?;
                form = form.part("file", part);
            }
            Some(AudioInput::Base64(base64)) => {
                form = form.text("file_base64", base64.clone());
            }
            None => {
                return Err(ZhipuError::InvalidArgument(
                    "audio input is required".to_string(),
                ));
            }
        }

        // Add optional fields
        if let Some(prompt) = &request.prompt {
            form = form.text("prompt", prompt.clone());
        }

        if let Some(hotwords) = &request.hotwords {
            let hotwords_json = serde_json::to_string(hotwords)
                .map_err(|e| ZhipuError::InvalidArgument(e.to_string()))?;
            form = form.text("hotwords", hotwords_json);
        }

        if let Some(stream) = request.stream {
            form = form.text("stream", stream.to_string());
        }

        if let Some(request_id) = &request.request_id {
            form = form.text("request_id", request_id.clone());
        }

        if let Some(user_id) = &request.user_id {
            form = form.text("user_id", user_id.clone());
        }

        Ok(form)
    }
}
