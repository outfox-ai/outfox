//! Speech synthesis implementation using WebSocket.

use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::handshake::client::generate_key;
use tokio_tungstenite::tungstenite::http::Request;
use tokio_tungstenite::tungstenite::protocol::Message;

use crate::Client;
use crate::error::{DoubaoError, Result};
use crate::spec::tts::{
    Additions, AudioParams, CreateSpeechRequest, CreateSpeechResponse, EVENT_CONNECTION_STARTED,
    EVENT_FINISH_CONNECTION, EVENT_FINISH_SESSION, EVENT_SESSION_FINISHED, EVENT_SESSION_STARTED,
    EVENT_START_CONNECTION, EVENT_START_SESSION, EVENT_TASK_REQUEST, EVENT_TTS_RESPONSE,
    EVENT_TTS_SENTENCE_END, EVENT_TTS_SENTENCE_START, NAMESPACE_BIDIRECTIONAL_TTS,
    StartSessionPayload, TaskRequestPayload, TtsRequestParams, UserInfo, build_event_frame,
    extract_audio_from_frame, parse_event,
};

/// Speech synthesis API.
pub struct Speech<'c> {
    client: &'c Client,
}

impl<'c> Speech<'c> {
    /// Create a new Speech API.
    pub(crate) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Create speech from text using the bidirectional WebSocket API.
    ///
    /// # Errors
    ///
    /// Returns an error if the WebSocket connection fails or the API returns an error.
    pub async fn create(&self, request: CreateSpeechRequest) -> Result<CreateSpeechResponse> {
        let config = self.client.config();
        let connect_id = uuid::Uuid::new_v4().to_string();

        println!("[TTS] Creating speech request...");
        println!("[TTS] connect_id={}", connect_id);
        println!("[TTS] tts_ws_base={}", config.tts_ws_base());
        println!("[TTS] app_id={}", config.app_id());
        println!("[TTS] resource_id={}", config.resource_id());
        println!("[TTS] authorization={}", config.authorization());

        // Build WebSocket request with authentication headers (following reference implementation)
        let ws_request = Request::builder()
            .uri(config.tts_ws_base())
            .header("Host", "openspeech.bytedance.com")
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Version", "13")
            .header("Sec-WebSocket-Key", generate_key())
            .header("Authorization", config.authorization())
            .header("X-Api-App-Key", config.app_id())
            .header("X-Api-Access-Key", config.access_token())
            .header("X-Api-Resource-Id", config.resource_id())
            .header("X-Api-Connect-Id", &connect_id)
            .body(())
            .map_err(|e| DoubaoError::Protocol(format!("failed to build request: {}", e)))?;

        println!("[TTS] Connecting to WebSocket...");

        // Connect to WebSocket
        let (ws_stream, response) = connect_async(ws_request).await?;

        println!("[TTS] WebSocket connected successfully");

        // Log response headers for debugging
        if let Some(logid) = response.headers().get("X-Tt-Logid") {
            println!("[TTS] X-Tt-Logid: {:?}", logid);
            tracing::debug!("X-Tt-Logid: {:?}", logid);
        }

        let (mut write, mut read) = ws_stream.split();

        // Generate unique IDs
        let session_id = uuid::Uuid::new_v4().to_string();
        let user_id = uuid::Uuid::new_v4().to_string();

        // Extract request parameters
        let format = request.format.unwrap_or_default();
        let sample_rate = request.sample_rate.unwrap_or(24000);
        let speech_rate = request.speech_rate.unwrap_or(0);
        let speaker = &request.speaker;
        let text = &request.text;

        // 1. Send StartConnection
        let start_conn_frame = build_event_frame(EVENT_START_CONNECTION, None, &json!({}));
        write.send(Message::Binary(start_conn_frame.into())).await?;
        tracing::debug!("Sent StartConnection");

        // Wait for ConnectionStarted
        Self::wait_for_event(&mut read, EVENT_CONNECTION_STARTED).await?;
        tracing::debug!("Received ConnectionStarted");

        // 2. Send StartSession (following reference implementation exactly)

        let additions = Additions::new(request.disable_markdown_filter.unwrap_or(false));

        let start_session_payload = StartSessionPayload {
            user: UserInfo {
                uid: user_id.clone(),
            },
            event: EVENT_START_SESSION,
            namespace: NAMESPACE_BIDIRECTIONAL_TTS.to_string(),
            req_params: TtsRequestParams {
                speaker: request.speaker.clone(),
                audio_params: Some(AudioParams {
                    format: Some(format),
                    sample_rate: Some(sample_rate),
                    speech_rate: request.speech_rate,
                    loudness_rate: request.loudness_rate,
                    pitch_rate: request.pitch_rate,
                    enable_timestamp: request.enable_timestamp,
                }),
                text: None,
                additions: Some(additions.to_json_string()),
            },
        };

        let start_session_frame = build_event_frame(
            EVENT_START_SESSION,
            Some(&session_id),
            &serde_json::to_value(&start_session_payload).map_err(|e| {
                DoubaoError::Protocol(format!("failed to serialize payload: {}", e))
            })?,
        );
        write
            .send(Message::Binary(start_session_frame.into()))
            .await?;
        tracing::debug!("Sent StartSession");

        // Wait for SessionStarted
        println!("[TTS] Waiting for SessionStarted...");
        Self::wait_for_event(&mut read, EVENT_SESSION_STARTED).await?;
        tracing::debug!("Received SessionStarted");

        let end_session_frame = build_event_frame(EVENT_SESSION_FINISHED, Some(&session_id), &json!({}));
        write
            .send(Message::Binary(end_session_frame.into()))
            .await?;
        // 3. Send TaskRequest with text (following reference implementation exactly)

        let task_payload = TaskRequestPayload {
            user: UserInfo { uid: user_id },
            event: EVENT_TASK_REQUEST,
            namespace: NAMESPACE_BIDIRECTIONAL_TTS.to_string(),
            req_params: TtsRequestParams {
                speaker: request.speaker,
                audio_params: Some(AudioParams {
                    format: Some(format),
                    sample_rate: Some(sample_rate),
                    speech_rate: request.speech_rate,
                    loudness_rate: request.loudness_rate,
                    pitch_rate: request.pitch_rate,
                    enable_timestamp: request.enable_timestamp,
                }),
                text: Some(request.text),
                additions: Some(additions.to_json_string()),
            },
        };

        // 4. Receive audio data
        let mut audio_data = Vec::new();
        loop {
            match read.next().await {
                Some(Ok(Message::Binary(data))) => {
                    if data.len() < 4 {
                        continue;
                    }

                    let event = parse_event(&data)
                        .ok_or_else(|| DoubaoError::Protocol("invalid frame".to_string()))?;

                    match event {
                        EVENT_TTS_RESPONSE | EVENT_TTS_SENTENCE_START | EVENT_TTS_SENTENCE_END => {
                            if let Some(audio) = extract_audio_from_frame(&data) {
                                audio_data.extend_from_slice(&audio);
                            }
                        }
                        EVENT_SESSION_FINISHED => {
                            tracing::debug!("Session finished");
                            println!("[TTS] Received EVENT_SESSION_FINISHED");
                            break;
                        }
                        _ => {
                            println!("[TTS] Received unknown event: {}  {}", event, data.len());
                            tracing::debug!("Received unknown event: {}", event);
                        }
                    }
                }
                Some(Ok(Message::Text(txt))) => {
                    tracing::warn!("Received unexpected text message: {}", txt);
                }
                Some(Ok(Message::Close(frame))) => {
                    tracing::debug!("WebSocket closed");
                    break;
                }
                Some(Ok(Message::Ping(data))) => {
                    println!("[TTS] Received Ping");
                    break;
                }
                Some(Ok(Message::Pong(data))) => {
                    println!("[TTS] Received Pong");
                    break;
                }
                Some(Ok(Message::Frame(_))) => {
                    println!("[TTS] Received raw frame");
                }
                Some(Err(e)) => {
                    tracing::error!("WebSocket error: {}", e);
                    return Err(e.into());
                }
                None => {
                    tracing::debug!("WebSocket stream ended");
                    break;
                }
            }
        }

        // 5. Send FinishSession
        let finish_session_frame =
            build_event_frame(EVENT_FINISH_SESSION, Some(&session_id), &json!({}));
        let _ = write
            .send(Message::Binary(finish_session_frame.into()))
            .await;
        println!("[TTS] Sent FinishSession");

        // 6. Send FinishConnection
        let finish_conn_frame = build_event_frame(EVENT_FINISH_CONNECTION, None, &json!({}));
        let _ = write.send(Message::Binary(finish_conn_frame.into())).await;

        tracing::info!("TTS completed, received {} bytes", audio_data.len());

        Ok(CreateSpeechResponse::new(
            Bytes::from(audio_data),
            format,
            sample_rate,
        ))
    }

    /// Wait for a specific event from the WebSocket stream.
    async fn wait_for_event<S>(
        read: &mut futures_util::stream::SplitStream<S>,
        expected_event: i32,
    ) -> Result<()>
    where
        S: futures_util::Stream<
                Item = std::result::Result<Message, tokio_tungstenite::tungstenite::Error>,
            > + Unpin,
    {
        println!("[TTS] wait_for_event: waiting for event {}", expected_event);
        while let Some(result) = read.next().await {
            match result {
                Ok(Message::Binary(data)) => {
                    println!(
                        "[TTS] wait_for_event: received binary message, len={}",
                        data.len()
                    );
                    if let Some(event) = parse_event(&data) {
                        println!(
                            "[TTS] wait_for_event: parsed event={}, expected={}",
                            event, expected_event
                        );
                        if event == expected_event {
                            println!("[TTS] wait_for_event: matched!");
                            return Ok(());
                        }
                    }
                }
                Ok(msg) => {
                    println!(
                        "[TTS] wait_for_event: received non-binary message: {:?}",
                        msg
                    );
                    continue;
                }
                Err(e) => {
                    println!("[TTS] wait_for_event: error: {}", e);
                    return Err(e.into());
                }
            }
        }
        println!(
            "[TTS] wait_for_event: stream ended without receiving expected event {}",
            expected_event
        );
        Err(DoubaoError::EventNotReceived {
            expected: expected_event,
        })
    }
}
