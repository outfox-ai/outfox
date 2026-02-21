//! HTTP Unidirectional TTS (v3 API) implementation.
//!
//! This module implements the v3 HTTP streaming API for TTS.
//! URL: https://openspeech.bytedance.com/api/v3/tts/unidirectional
//!
//! The text is sent in one POST request, and audio is streamed back as JSON chunks
//! with base64-encoded audio data.

use base64::Engine;
use bytes::Bytes;

use crate::Client;
use crate::error::{ApiError, DoubaoError, Result};
use crate::spec::tts::{
    AudioFormat, CreateSpeechRequest, CreateSpeechResponse, V3UniAudioParams, V3UniReqParams,
    V3UniRequest, V3UniStreamResponse, V3UniUser,
};

/// HTTP Unidirectional Speech synthesis API (v3 streaming).
pub struct SpeechHttpV3<'c> {
    client: &'c Client,
}

impl<'c> SpeechHttpV3<'c> {
    /// Create a new HTTP v3 Speech API.
    pub(crate) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Create speech from text using the v3 HTTP streaming API.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails or the API returns an error.
    pub async fn create(&self, request: CreateSpeechRequest) -> Result<CreateSpeechResponse> {
        let config = self.client.config();

        println!("[TTS-HTTP-V3] ==================== TTS Request Start ====================");
        println!("[TTS-HTTP-V3] url={}", config.tts_http_v3_base());
        println!("[TTS-HTTP-V3] app_id={}", config.app_id());
        println!("[TTS-HTTP-V3] resource_id={}", config.resource_id());
        println!("[TTS-HTTP-V3] speaker={}", request.speaker);

        // Generate unique user ID
        let uid = uuid::Uuid::new_v4().to_string();
        println!("[TTS-HTTP-V3] uid={}", uid);

        // Extract request parameters
        let format = request.format.unwrap_or_default();
        let sample_rate = request.sample_rate.unwrap_or(48000);
        let speech_rate = request.speech_rate.unwrap_or(0);

        let format_str = match format {
            AudioFormat::Mp3 => "mp3",
            AudioFormat::Pcm => "pcm",
            AudioFormat::Ogg => "ogg_opus",
            AudioFormat::Wav => "pcm", // V3 API doesn't support wav directly, use pcm
        };

        println!(
            "[TTS-HTTP-V3] format={}, sample_rate={}, speech_rate={}",
            format_str, sample_rate, speech_rate
        );
        println!("[TTS-HTTP-V3] text={}", request.text);

        // Build V3 request payload
        let v3_request = V3UniRequest {
            user: V3UniUser { uid },
            req_params: V3UniReqParams {
                text: request.text.clone(),
                speaker: request.speaker.clone(),
                audio_params: Some(V3UniAudioParams {
                    format: Some(format_str.to_string()),
                    sample_rate: Some(sample_rate),
                    bit_rate: None,
                    speech_rate: Some(speech_rate),
                }),
            },
        };

        // Create HTTP client
        let http_client = reqwest::Client::new();

        let payload = serde_json::to_value(&v3_request).map_err(|e| {
            DoubaoError::Protocol(format!("failed to serialize request: {}", e))
        })?;

        println!(
            "[TTS-HTTP-V3] Sending request: {}",
            serde_json::to_string_pretty(&payload).unwrap_or_default()
        );

        // Send POST request with required headers
        let response = http_client
            .post(config.tts_http_v3_base())
            .header("Content-Type", "application/json")
            .header("X-Api-App-Id", config.app_id())
            .header("X-Api-Access-Key", config.access_token())
            .header("X-Api-Resource-Id", config.resource_id())
            .json(&v3_request)
            .send()
            .await
            .map_err(|e| DoubaoError::HttpError(e.to_string()))?;

        println!("[TTS-HTTP-V3] Response status: {}", response.status());

        // Check HTTP status
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            println!("[TTS-HTTP-V3] Error response body: {}", body);
            return Err(DoubaoError::HttpError(format!(
                "HTTP error {}: {}",
                status, body
            )));
        }

        // Read streaming response
        let response_text = response.text().await.map_err(|e| {
            DoubaoError::HttpError(format!("failed to read response body: {}", e))
        })?;

        println!("[TTS-HTTP-V3] Response body length: {} bytes", response_text.len());

        // Parse streaming JSON responses (one JSON per line)
        let mut audio_data = Vec::new();
        let mut total_words = 0u32;

        for line in response_text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            println!("[TTS-HTTP-V3] Parsing line: {}", &line[..std::cmp::min(100, line.len())]);

            let chunk: V3UniStreamResponse = serde_json::from_str(line).map_err(|e| {
                DoubaoError::Protocol(format!("failed to parse response chunk: {} - line: {}", e, line))
            })?;

            match chunk.code {
                0 => {
                    // Audio data chunk
                    if let Some(data) = &chunk.data {
                        let decoded = base64::engine::general_purpose::STANDARD
                            .decode(data)
                            .map_err(|e| {
                                DoubaoError::Protocol(format!("failed to decode audio data: {}", e))
                            })?;
                        println!(
                            "[TTS-HTTP-V3] Audio chunk: {} bytes decoded",
                            decoded.len()
                        );
                        audio_data.extend_from_slice(&decoded);
                    }
                }
                20000000 => {
                    // Synthesis complete
                    if let Some(usage) = &chunk.usage {
                        total_words = usage.text_words;
                        println!(
                            "[TTS-HTTP-V3] Synthesis complete: {} words processed",
                            total_words
                        );
                    }
                }
                _ => {
                    // Error response
                    println!(
                        "[TTS-HTTP-V3] Error response: code={}, message={}",
                        chunk.code, chunk.message
                    );
                    return Err(DoubaoError::ApiError(ApiError {
                        code: Some(chunk.code),
                        message: chunk.message,
                        details: None,
                    }));
                }
            }
        }

        println!("[TTS-HTTP-V3] ==================== TTS Complete ====================");
        println!(
            "[TTS-HTTP-V3] TTS completed, received {} total audio bytes, {} words",
            audio_data.len(),
            total_words
        );
        tracing::info!(
            "TTS completed, received {} bytes, {} words",
            audio_data.len(),
            total_words
        );

        Ok(CreateSpeechResponse::new(
            Bytes::from(audio_data),
            format,
            sample_rate,
        ))
    }
}
