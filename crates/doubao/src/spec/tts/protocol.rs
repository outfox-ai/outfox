//! WebSocket protocol constants and types for Doubao TTS APIs.
//!
//! This module contains protocol definitions for:
//! - V3 Bidirectional WebSocket API (binary protocol)
//! - V3 Unidirectional APIs use standard JSON over HTTP/WebSocket

// =============================================================================
// V3 Bidirectional Protocol Constants (Binary WebSocket)
// =============================================================================

/// Protocol version: v1 with 4-byte header.
pub const PROTOCOL_VERSION: u8 = 0x11;

/// Message type: Full client request with event.
pub const MSG_TYPE_FULL_CLIENT: u8 = 0x14;

/// Message type: Full server response with event.
pub const MSG_TYPE_FULL_SERVER: u8 = 0x94;

/// Message type: Audio-only response with event.
pub const MSG_TYPE_AUDIO_ONLY: u8 = 0xB4;

/// Serialization method: JSON.
pub const SERIALIZATION_JSON: u8 = 0x10;

/// No compression.
pub const NO_COMPRESSION: u8 = 0x00;

/// Reserved byte (always 0).
pub const RESERVED: u8 = 0x00;

/// Event: Start connection.
pub const EVENT_START_CONNECTION: i32 = 1;

/// Event: Connection started (response).
pub const EVENT_CONNECTION_STARTED: i32 = 50;

/// Event: Start session.
pub const EVENT_START_SESSION: i32 = 100;

/// Event: Session started (response).
pub const EVENT_SESSION_STARTED: i32 = 150;

/// Event: Task request (send text for TTS).
pub const EVENT_TASK_REQUEST: i32 = 200;

/// Event: TTS sentence start.
pub const EVENT_TTS_SENTENCE_START: i32 = 350;

/// Event: TTS sentence end.
pub const EVENT_TTS_SENTENCE_END: i32 = 351;

/// Event: TTS response (audio data).
pub const EVENT_TTS_RESPONSE: i32 = 352;

/// Event: Session finished (response).
pub const EVENT_SESSION_FINISHED: i32 = 152;

/// Event: Finish session.
pub const EVENT_FINISH_SESSION: i32 = 102;

/// Event: Finish connection.
pub const EVENT_FINISH_CONNECTION: i32 = 2;

/// Namespace for bidirectional TTS.
pub const NAMESPACE_BIDIRECTIONAL_TTS: &str = "BidirectionalTTS";

// =============================================================================
// V3 Bidirectional Protocol Functions (Binary WebSocket)
// =============================================================================

/// Build a protocol frame with the given event and payload.
///
/// Frame format:
/// - Header (4 bytes): [protocol_version, msg_type, serialization|compression, reserved]
/// - Event number (4 bytes, big-endian)
/// - Session ID length (4 bytes, big-endian) + Session ID bytes (if provided)
/// - Payload length (4 bytes, big-endian) + Payload bytes
#[must_use]
pub fn build_event_frame(
    event: i32,
    session_id: Option<&str>,
    payload: &serde_json::Value,
) -> Vec<u8> {
    let mut frame = Vec::new();

    // Header (4 bytes)
    frame.push(PROTOCOL_VERSION);
    frame.push(MSG_TYPE_FULL_CLIENT);
    frame.push(SERIALIZATION_JSON | NO_COMPRESSION);
    frame.push(RESERVED);

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

/// Parse the event number from a binary frame.
///
/// Returns `None` if the frame is too short.
#[must_use]
pub fn parse_event(data: &[u8]) -> Option<i32> {
    if data.len() < 8 {
        return None;
    }
    let event = i32::from_be_bytes([data[4], data[5], data[6], data[7]]);
    Some(event)
}

/// Extract audio data from an audio-only response frame.
///
/// Returns `None` if the frame is not an audio-only response or is malformed.
#[must_use]
pub fn extract_audio_from_frame(data: &[u8]) -> Option<Vec<u8>> {
    if data.len() < 4 {
        return None;
    }

    let msg_type = data[1];
    // Audio-only response (0xB4)
    if msg_type == MSG_TYPE_AUDIO_ONLY {
        // Header (4 bytes) + Event (4 bytes) + Session ID length (4 bytes)
        if data.len() < 12 {
            return None;
        }

        let session_id_len = u32::from_be_bytes([data[8], data[9], data[10], data[11]]) as usize;
        let audio_offset = 12 + session_id_len + 4; // +4 for payload size field

        if data.len() > audio_offset {
            let audio_data = data[audio_offset..].to_vec();
            return Some(audio_data);
        } else {
            println!(
                "[TTS] No audio data: data_len={} <= audio_offset={}",
                data.len(),
                audio_offset
            );
        }
    } else if msg_type == MSG_TYPE_FULL_SERVER {
        println!("[TTS] MSG_TYPE_FULL_SERVER detected (not extracting audio from this type)");
    } else {
        println!("[TTS] Unknown msg_type: 0x{:02X}", msg_type);
    }

    None
}

/// Check if a frame is a full server response.
#[must_use]
pub fn is_full_server_response(data: &[u8]) -> bool {
    data.len() >= 2 && data[1] == MSG_TYPE_FULL_SERVER
}

/// Check if a frame is an audio-only response.
#[must_use]
pub fn is_audio_only_response(data: &[u8]) -> bool {
    data.len() >= 2 && data[1] == MSG_TYPE_AUDIO_ONLY
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_event_frame() {
        let payload = serde_json::json!({});
        let frame = build_event_frame(EVENT_START_CONNECTION, None, &payload);

        // Check header
        assert_eq!(frame[0], PROTOCOL_VERSION);
        assert_eq!(frame[1], MSG_TYPE_FULL_CLIENT);
        assert_eq!(frame[2], SERIALIZATION_JSON | NO_COMPRESSION);
        assert_eq!(frame[3], RESERVED);

        // Check event
        let event = i32::from_be_bytes([frame[4], frame[5], frame[6], frame[7]]);
        assert_eq!(event, EVENT_START_CONNECTION);
    }

    #[test]
    fn test_parse_event() {
        let payload = serde_json::json!({});
        let frame = build_event_frame(EVENT_START_SESSION, Some("test-session"), &payload);

        let event = parse_event(&frame);
        assert_eq!(event, Some(EVENT_START_SESSION));
    }

    #[test]
    fn test_parse_event_too_short() {
        let data = [0u8; 4];
        assert_eq!(parse_event(&data), None);
    }
}
