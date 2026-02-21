//! ASR protocol constants for Doubao ASR v3 API.

/// Base URL for ASR API.
pub const ASR_API_BASE: &str = "https://openspeech.bytedance.com/api/v3/auc/bigmodel";

/// Submit task endpoint (standard version).
pub const ASR_SUBMIT_PATH: &str = "/submit";

/// Query result endpoint (standard version).
pub const ASR_QUERY_PATH: &str = "/query";

/// Flash recognition endpoint (turbo version).
pub const ASR_FLASH_PATH: &str = "/recognize/flash";

/// Resource ID for standard ASR.
pub const RESOURCE_ID_BIGASR: &str = "volc.bigasr.auc";

/// Resource ID for SeedASR.
pub const RESOURCE_ID_SEEDASR: &str = "volc.seedasr.auc";

/// Resource ID for turbo/flash ASR.
pub const RESOURCE_ID_BIGASR_TURBO: &str = "volc.bigasr.auc_turbo";

/// Status code: Success.
pub const STATUS_SUCCESS: i32 = 20000000;

/// Status code: Processing.
pub const STATUS_PROCESSING: i32 = 20000001;

/// Status code: In queue.
pub const STATUS_IN_QUEUE: i32 = 20000002;

/// Status code: Silent audio.
pub const STATUS_SILENT: i32 = 20000003;

/// Status code: Invalid parameter.
pub const STATUS_INVALID_PARAM: i32 = 45000001;

/// Status code: Empty audio.
pub const STATUS_EMPTY_AUDIO: i32 = 45000002;

/// Status code: Invalid audio format.
pub const STATUS_INVALID_FORMAT: i32 = 45000151;

/// Status code: Server busy.
pub const STATUS_SERVER_BUSY: i32 = 55000031;

/// Header name for App Key.
pub const HEADER_APP_KEY: &str = "X-Api-App-Key";

/// Header name for Access Key.
pub const HEADER_ACCESS_KEY: &str = "X-Api-Access-Key";

/// Header name for Resource ID.
pub const HEADER_RESOURCE_ID: &str = "X-Api-Resource-Id";

/// Header name for Request ID.
pub const HEADER_REQUEST_ID: &str = "X-Api-Request-Id";

/// Header name for Sequence.
pub const HEADER_SEQUENCE: &str = "X-Api-Sequence";

/// Header name for Status Code (response).
pub const HEADER_STATUS_CODE: &str = "X-Api-Status-Code";

/// Header name for Message (response).
pub const HEADER_MESSAGE: &str = "X-Api-Message";

/// Header name for Log ID (response).
pub const HEADER_LOG_ID: &str = "X-Tt-Logid";

/// WebSocket URL for streaming ASR.
pub const ASR_WS_URL: &str = "wss://openspeech.bytedance.com/api/v3/sauc/bigmodel";

/// Streaming protocol version.
pub const STREAMING_PROTOCOL_VERSION: u8 = 0x11;

/// Message type: Full client request.
pub const STREAMING_MSG_FULL_CLIENT: u8 = 0x14;

/// Message type: Full server response.
pub const STREAMING_MSG_FULL_SERVER: u8 = 0x94;

/// Message type: Audio only client.
pub const STREAMING_MSG_AUDIO_ONLY_CLIENT: u8 = 0x24;

/// Serialization: JSON.
pub const STREAMING_SERIAL_JSON: u8 = 0x10;

/// Compression: None.
pub const STREAMING_COMPRESS_NONE: u8 = 0x00;

/// Compression: Gzip.
pub const STREAMING_COMPRESS_GZIP: u8 = 0x01;

/// Event: Start connection.
pub const STREAMING_EVENT_START_CONNECTION: i32 = 1;

/// Event: Finish connection.
pub const STREAMING_EVENT_FINISH_CONNECTION: i32 = 2;

/// Event: Connection started.
pub const STREAMING_EVENT_CONNECTION_STARTED: i32 = 50;

/// Event: Connection failed.
pub const STREAMING_EVENT_CONNECTION_FAILED: i32 = 51;

/// Event: Start session.
pub const STREAMING_EVENT_START_SESSION: i32 = 100;

/// Event: Finish session.
pub const STREAMING_EVENT_FINISH_SESSION: i32 = 102;

/// Event: Session started.
pub const STREAMING_EVENT_SESSION_STARTED: i32 = 150;

/// Event: Session finished.
pub const STREAMING_EVENT_SESSION_FINISHED: i32 = 152;

/// Event: Session failed.
pub const STREAMING_EVENT_SESSION_FAILED: i32 = 153;

/// Event: Task request.
pub const STREAMING_EVENT_TASK_REQUEST: i32 = 200;

/// Event: ASR result.
pub const STREAMING_EVENT_ASR_RESULT: i32 = 350;
