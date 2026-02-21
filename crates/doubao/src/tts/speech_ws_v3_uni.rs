//! WebSocket Unidirectional TTS (v3 API) implementation.
//!
//! This module implements the v3 unidirectional WebSocket streaming API for TTS.
//! URL: wss://openspeech.bytedance.com/api/v3/tts/unidirectional
//!
//! The text is sent as JSON, and audio is streamed back as JSON chunks
//! with base64-encoded audio data.

use base64::Engine;
use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

use crate::Client;
use crate::error::{ApiError, DoubaoError, Result};
use crate::spec::tts::{
    AudioFormat, CreateSpeechRequest, CreateSpeechResponse, V3UniAudioParams, V3UniReqParams,
    V3UniRequest, V3UniStreamResponse, V3UniUser,
};

/// WebSocket Unidirectional Speech synthesis API (v3 streaming).
pub struct SpeechWsV3Uni<'c> {
    client: &'c Client,
}

impl<'c> SpeechWsV3Uni<'c> {
    /// Create a new WebSocket v3 unidirectional Speech API.
    pub(crate) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Create speech from text using the v3 unidirectional WebSocket streaming API.
    ///
    /// # Errors
    ///
    /// Returns an error if the WebSocket connection fails or the API returns an error.
    pub async fn create(&self, request: CreateSpeechRequest) -> Result<CreateSpeechResponse> {
        let config = self.client.config();

        println!("[TTS-WS-V3-UNI] ==================== TTS Request Start ====================");
        println!("[TTS-WS-V3-UNI] url={}", config.tts_ws_v3_uni_base());
        println!("[TTS-WS-V3-UNI] app_id={}", config.app_id());
        println!("[TTS-WS-V3-UNI] resource_id={}", config.resource_id());
        println!("[TTS-WS-V3-UNI] speaker={}", request.speaker);

        // Generate unique user ID
        let uid = uuid::Uuid::new_v4().to_string();
        println!("[TTS-WS-V3-UNI] uid={}", uid);

        // Extract request parameters
        let format = request.format.unwrap_or_default();
        let sample_rate = request.sample_rate.unwrap_or(24000);
        let speech_rate = request.speech_rate.unwrap_or(0);

        let format_str = match format {
            AudioFormat::Mp3 => "mp3",
            AudioFormat::Pcm => "pcm",
            AudioFormat::Ogg => "ogg_opus",
            AudioFormat::Wav => "pcm", // V3 API doesn't support wav directly, use pcm
        };

        println!(
            "[TTS-WS-V3-UNI] format={}, sample_rate={}, speech_rate={}",
            format_str, sample_rate, speech_rate
        );
        println!("[TTS-WS-V3-UNI] text={}", request.text);

        // Build WebSocket URL with query parameters for authentication
        let ws_url = config.tts_ws_v3_uni_base();

        println!("[TTS-WS-V3-UNI] Connecting to WebSocket: {}", ws_url);

        // Connect to WebSocket with required headers
        let ws_request = tokio_tungstenite::tungstenite::http::Request::builder()
            .uri(ws_url)
            .header("Host", "openspeech.bytedance.com")
            .header("X-Api-App-Id", config.app_id())
            .header("X-Api-Access-Key", config.access_token())
            .header("X-Api-Resource-Id", config.resource_id())
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Version", "13")
            .header(
                "Sec-WebSocket-Key",
                tokio_tungstenite::tungstenite::handshake::client::generate_key(),
            )
            .body(())
            .map_err(|e| DoubaoError::Protocol(format!("failed to build request: {}", e)))?;

        let (ws_stream, response) = connect_async(ws_request).await?;

        println!("[TTS-WS-V3-UNI] WebSocket connected successfully");

        // Log response headers for debugging
        if let Some(logid) = response.headers().get("X-Tt-Logid") {
            println!("[TTS-WS-V3-UNI] X-Tt-Logid: {:?}", logid);
            tracing::debug!("X-Tt-Logid: {:?}", logid);
        }

        let (mut write, mut read) = ws_stream.split();

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

        // Serialize and send request as JSON text
        let payload = serde_json::to_string(&v3_request).map_err(|e| {
            DoubaoError::Protocol(format!("failed to serialize request: {}", e))
        })?;

        println!("[TTS-WS-V3-UNI] Sending request: {}", payload);

        write.send(Message::Text(payload.into())).await?;
        println!("[TTS-WS-V3-UNI] Request sent");

        // Receive audio data
        println!("[TTS-WS-V3-UNI] Receiving audio data...");
        let mut audio_data = Vec::new();
        let mut total_words = 0u32;

        loop {
            match read.next().await {
                Some(Ok(Message::Text(txt))) => {
                    let txt_str: &str = txt.as_ref();
                    println!(
                        "[TTS-WS-V3-UNI] Received text message: {}",
                        &txt_str[..std::cmp::min(200, txt_str.len())]
                    );

                    // Parse JSON response
                    let chunk: V3UniStreamResponse =
                        serde_json::from_str(txt_str).map_err(|e| {
                            DoubaoError::Protocol(format!(
                                "failed to parse response: {} - text: {}",
                                e, txt_str
                            ))
                        })?;

                    match chunk.code {
                        0 => {
                            // Audio data chunk
                            if let Some(data) = &chunk.data {
                                let decoded = base64::engine::general_purpose::STANDARD
                                    .decode(data)
                                    .map_err(|e| {
                                        DoubaoError::Protocol(format!(
                                            "failed to decode audio data: {}",
                                            e
                                        ))
                                    })?;
                                println!(
                                    "[TTS-WS-V3-UNI] Audio chunk: {} bytes decoded",
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
                                    "[TTS-WS-V3-UNI] Synthesis complete: {} words processed",
                                    total_words
                                );
                            }
                            break;
                        }
                        _ => {
                            // Error response
                            println!(
                                "[TTS-WS-V3-UNI] Error response: code={}, message={}",
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
                Some(Ok(Message::Binary(data))) => {
                    println!(
                        "[TTS-WS-V3-UNI] Received binary message, len={}",
                        data.len()
                    );
                    // Try to parse as JSON in case the server sends binary JSON
                    let txt = String::from_utf8_lossy(&data);
                    if let Ok(chunk) = serde_json::from_str::<V3UniStreamResponse>(&txt) {
                        match chunk.code {
                            0 => {
                                if let Some(data_str) = &chunk.data {
                                    let decoded = base64::engine::general_purpose::STANDARD
                                        .decode(data_str)
                                        .map_err(|e| {
                                            DoubaoError::Protocol(format!(
                                                "failed to decode audio data: {}",
                                                e
                                            ))
                                        })?;
                                    audio_data.extend_from_slice(&decoded);
                                }
                            }
                            20000000 => {
                                if let Some(usage) = &chunk.usage {
                                    total_words = usage.text_words;
                                }
                                break;
                            }
                            _ => {
                                return Err(DoubaoError::ApiError(ApiError {
                                    code: Some(chunk.code),
                                    message: chunk.message,
                                    details: None,
                                }));
                            }
                        }
                    }
                }
                Some(Ok(Message::Close(frame))) => {
                    println!("[TTS-WS-V3-UNI] WebSocket closed: {:?}", frame);
                    break;
                }
                Some(Ok(Message::Ping(_))) => {
                    println!("[TTS-WS-V3-UNI] Received ping");
                }
                Some(Ok(Message::Pong(_))) => {
                    println!("[TTS-WS-V3-UNI] Received pong");
                }
                Some(Ok(Message::Frame(_))) => {
                    println!("[TTS-WS-V3-UNI] Received raw frame");
                }
                Some(Err(e)) => {
                    println!("[TTS-WS-V3-UNI] WebSocket error: {}", e);
                    tracing::error!("WebSocket error: {}", e);
                    return Err(e.into());
                }
                None => {
                    println!("[TTS-WS-V3-UNI] WebSocket stream ended");
                    break;
                }
            }
        }

        println!("[TTS-WS-V3-UNI] ==================== TTS Complete ====================");
        println!(
            "[TTS-WS-V3-UNI] TTS completed, received {} total audio bytes, {} words",
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
