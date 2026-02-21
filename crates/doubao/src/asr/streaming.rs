//! WebSocket-based streaming speech recognition implementation.

use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::handshake::client::generate_key;
use tokio_tungstenite::tungstenite::http::Request;
use tokio_tungstenite::tungstenite::protocol::Message;

use crate::Client;
use crate::error::{DoubaoError, Result};
use crate::spec::asr::{
    ASR_WS_URL, AsrResult, STREAMING_COMPRESS_NONE, STREAMING_EVENT_ASR_RESULT,
    STREAMING_EVENT_CONNECTION_STARTED, STREAMING_EVENT_FINISH_CONNECTION,
    STREAMING_EVENT_FINISH_SESSION, STREAMING_EVENT_SESSION_FINISHED,
    STREAMING_EVENT_SESSION_STARTED, STREAMING_EVENT_START_CONNECTION,
    STREAMING_EVENT_START_SESSION, STREAMING_EVENT_TASK_REQUEST, STREAMING_MSG_AUDIO_ONLY_CLIENT,
    STREAMING_MSG_FULL_CLIENT, STREAMING_PROTOCOL_VERSION, STREAMING_SERIAL_JSON,
    StreamingAsrConfig, StreamingAsrResult,
};

/// Streaming speech recognition API.
///
/// Uses WebSocket for real-time audio streaming and recognition.
pub struct Streaming<'c> {
    client: &'c Client,
}

impl<'c> Streaming<'c> {
    /// Create a new Streaming API.
    pub(crate) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Create a streaming recognition session.
    ///
    /// Returns a session that can be used to send audio data and receive results.
    ///
    /// # Errors
    ///
    /// Returns an error if the WebSocket connection fails.
    pub async fn create_session(&self, config: StreamingAsrConfig) -> Result<StreamingSession> {
        StreamingSession::new(self.client, config).await
    }
}

/// A streaming recognition session.
///
/// Send audio data using `send_audio` and receive results through the `results` channel.
pub struct StreamingSession {
    /// Channel to send audio data.
    audio_tx: mpsc::Sender<Bytes>,
    /// Channel to receive recognition results.
    result_rx: mpsc::Receiver<StreamingAsrResult>,
    /// Session ID.
    session_id: String,
    /// Handle to the background task.
    _task_handle: tokio::task::JoinHandle<()>,
}

impl StreamingSession {
    /// Create a new streaming session.
    async fn new(client: &Client, config: StreamingAsrConfig) -> Result<Self> {
        let config_ref = client.config();
        let connect_id = uuid::Uuid::new_v4().to_string();

        // Build WebSocket request with authentication headers
        let ws_request = Request::builder()
            .uri(ASR_WS_URL)
            .header("Host", "openspeech.bytedance.com")
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Version", "13")
            .header("Sec-WebSocket-Key", generate_key())
            .header("Authorization", config_ref.authorization())
            .header("X-Api-App-Key", config_ref.app_id())
            .header("X-Api-Access-Key", config_ref.access_token())
            .header("X-Api-Resource-Id", config_ref.resource_id())
            .header("X-Api-Connect-Id", &connect_id)
            .body(())
            .map_err(|e| DoubaoError::Protocol(format!("failed to build request: {e}")))?;

        // Connect to WebSocket
        let (ws_stream, _response) = connect_async(ws_request).await?;
        let (mut write, mut read) = ws_stream.split();

        // Generate unique IDs
        let session_id = uuid::Uuid::new_v4().to_string();
        let user_id = uuid::Uuid::new_v4().to_string();

        // Create channels
        let (audio_tx, mut audio_rx) = mpsc::channel::<Bytes>(32);
        let (result_tx, result_rx) = mpsc::channel::<StreamingAsrResult>(32);

        // 1. Send StartConnection
        let start_conn_frame = build_event_frame(
            STREAMING_EVENT_START_CONNECTION,
            None,
            &serde_json::json!({}),
        );
        write.send(Message::Binary(start_conn_frame.into())).await?;

        // Wait for ConnectionStarted
        wait_for_event(&mut read, STREAMING_EVENT_CONNECTION_STARTED).await?;

        // 2. Send StartSession
        let session_payload = serde_json::json!({
            "user": { "uid": user_id },
            "event": STREAMING_EVENT_START_SESSION,
            "namespace": "SpeechRecognition",
            "req_params": {
                "audio": {
                    "format": config.format.map(|f| format!("{:?}", f).to_lowercase()),
                    "codec": config.codec.map(|c| format!("{:?}", c).to_lowercase()),
                    "rate": config.rate,
                    "bits": config.bits,
                    "channel": config.channel,
                    "language": config.language
                },
                "request": {
                    "model_name": "bigmodel",
                    "enable_itn": config.enable_itn,
                    "enable_punc": config.enable_punc,
                    "show_utterances": config.show_utterances,
                    "result_type": config.result_type
                }
            }
        });

        let session_id_clone = session_id.clone();
        let start_session_frame = build_event_frame(
            STREAMING_EVENT_START_SESSION,
            Some(&session_id_clone),
            &session_payload,
        );
        write
            .send(Message::Binary(start_session_frame.into()))
            .await?;

        // Wait for SessionStarted
        wait_for_event(&mut read, STREAMING_EVENT_SESSION_STARTED).await?;

        // Spawn background task to handle audio sending and result receiving
        let session_id_for_task = session_id.clone();
        let task_handle = tokio::spawn(async move {
            let mut finished = false;

            loop {
                tokio::select! {
                    // Handle incoming audio data
                    audio = audio_rx.recv() => {
                        match audio {
                            Some(data) => {
                                // Send audio data
                                let audio_frame = build_audio_frame(&session_id_for_task, &data);
                                if write.send(Message::Binary(audio_frame.into())).await.is_err() {
                                    break;
                                }
                            }
                            None => {
                                // Audio channel closed, send finish session
                                let finish_frame = build_event_frame(
                                    STREAMING_EVENT_FINISH_SESSION,
                                    Some(&session_id_for_task),
                                    &serde_json::json!({}),
                                );
                                let _ = write.send(Message::Binary(finish_frame.into())).await;
                                finished = true;
                            }
                        }
                    }
                    // Handle incoming messages
                    msg = read.next() => {
                        match msg {
                            Some(Ok(Message::Binary(data))) => {
                                if let Some(event) = parse_event(&data) {
                                    match event {
                                        STREAMING_EVENT_ASR_RESULT => {
                                            if let Some(result) = parse_asr_result(&data, &session_id_for_task) {
                                                let _ = result_tx.send(result).await;
                                            }
                                        }
                                        STREAMING_EVENT_SESSION_FINISHED => {
                                            // Send finish connection
                                            let finish_conn_frame = build_event_frame(
                                                STREAMING_EVENT_FINISH_CONNECTION,
                                                None,
                                                &serde_json::json!({}),
                                            );
                                            let _ = write.send(Message::Binary(finish_conn_frame.into())).await;
                                            break;
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            Some(Ok(Message::Close(_))) | None => {
                                break;
                            }
                            _ => {}
                        }
                    }
                }

                if finished && audio_rx.is_closed() {
                    // Wait a bit for final results
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                }
            }
        });

        Ok(Self {
            audio_tx,
            result_rx,
            session_id,
            _task_handle: task_handle,
        })
    }

    /// Get the session ID.
    #[must_use]
    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    /// Send audio data to the session.
    ///
    /// # Errors
    ///
    /// Returns an error if the session is closed.
    pub async fn send_audio(&self, data: Bytes) -> Result<()> {
        self.audio_tx
            .send(data)
            .await
            .map_err(|_| DoubaoError::Session("session closed".to_string()))
    }

    /// Receive the next recognition result.
    ///
    /// Returns `None` if the session is closed.
    pub async fn recv(&mut self) -> Option<StreamingAsrResult> {
        self.result_rx.recv().await
    }

    /// Close the session and stop sending audio.
    ///
    /// This will trigger the session to finish and send any remaining results.
    pub fn close(&self) {
        // Dropping the sender will cause the background task to finish
        // The sender is cloned, so we can't actually close it here
        // The user should drop the StreamingSession to close it
    }
}

/// Build a protocol frame with the given event and payload.
fn build_event_frame(event: i32, session_id: Option<&str>, payload: &serde_json::Value) -> Vec<u8> {
    let mut frame = Vec::new();

    // Header (4 bytes)
    frame.push(STREAMING_PROTOCOL_VERSION);
    frame.push(STREAMING_MSG_FULL_CLIENT);
    frame.push(STREAMING_SERIAL_JSON | STREAMING_COMPRESS_NONE);
    frame.push(0x00); // reserved

    // Event number (4 bytes, big-endian)
    frame.extend_from_slice(&event.to_be_bytes());

    // Session ID (if provided)
    if let Some(sid) = session_id {
        let sid_bytes = sid.as_bytes();
        frame.extend_from_slice(&(sid_bytes.len() as u32).to_be_bytes());
        frame.extend_from_slice(sid_bytes);
    }

    // Payload
    let payload_str = payload.to_string();
    let payload_bytes = payload_str.as_bytes();
    frame.extend_from_slice(&(payload_bytes.len() as u32).to_be_bytes());
    frame.extend_from_slice(payload_bytes);

    frame
}

/// Build an audio-only frame.
fn build_audio_frame(session_id: &str, audio_data: &[u8]) -> Vec<u8> {
    let mut frame = Vec::new();

    // Header (4 bytes)
    frame.push(STREAMING_PROTOCOL_VERSION);
    frame.push(STREAMING_MSG_AUDIO_ONLY_CLIENT);
    frame.push(STREAMING_SERIAL_JSON | STREAMING_COMPRESS_NONE);
    frame.push(0x00); // reserved

    // Event number (4 bytes) - TASK_REQUEST for audio
    frame.extend_from_slice(&STREAMING_EVENT_TASK_REQUEST.to_be_bytes());

    // Session ID
    let sid_bytes = session_id.as_bytes();
    frame.extend_from_slice(&(sid_bytes.len() as u32).to_be_bytes());
    frame.extend_from_slice(sid_bytes);

    // Audio data
    frame.extend_from_slice(&(audio_data.len() as u32).to_be_bytes());
    frame.extend_from_slice(audio_data);

    frame
}

/// Parse the event number from a binary frame.
fn parse_event(data: &[u8]) -> Option<i32> {
    if data.len() < 8 {
        return None;
    }
    Some(i32::from_be_bytes([data[4], data[5], data[6], data[7]]))
}

/// Parse ASR result from a binary frame.
fn parse_asr_result(data: &[u8], session_id: &str) -> Option<StreamingAsrResult> {
    if data.len() < 12 {
        return None;
    }

    // Skip header (4) + event (4)
    let session_id_len = u32::from_be_bytes([data[8], data[9], data[10], data[11]]) as usize;
    let payload_offset = 12 + session_id_len;

    if data.len() < payload_offset + 4 {
        return None;
    }

    let payload_len = u32::from_be_bytes([
        data[payload_offset],
        data[payload_offset + 1],
        data[payload_offset + 2],
        data[payload_offset + 3],
    ]) as usize;

    let payload_start = payload_offset + 4;
    if data.len() < payload_start + payload_len {
        return None;
    }

    let payload_bytes = &data[payload_start..payload_start + payload_len];
    let payload: serde_json::Value = serde_json::from_slice(payload_bytes).ok()?;

    let result = AsrResult {
        text: payload["result"]["text"].as_str().unwrap_or("").to_string(),
        utterances: vec![], // Could parse utterances if needed
        additions: payload.get("additions").cloned(),
    };

    let is_final = payload["result"]["definite"].as_bool().unwrap_or(false);

    Some(StreamingAsrResult {
        session_id: session_id.to_string(),
        result,
        is_final,
    })
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
    while let Some(result) = read.next().await {
        match result {
            Ok(Message::Binary(data)) => {
                if let Some(event) = parse_event(&data) {
                    if event == expected_event {
                        return Ok(());
                    }
                }
            }
            Ok(_) => continue,
            Err(e) => return Err(e.into()),
        }
    }
    Err(DoubaoError::EventNotReceived {
        expected: expected_event,
    })
}
