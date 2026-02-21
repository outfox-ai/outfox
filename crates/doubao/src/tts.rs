//! TTS (Text-to-Speech) API implementation.
//!
//! This module provides access to multiple TTS APIs:
//!
//! - **Bidirectional WebSocket (v3)**: Full-duplex streaming TTS with session management.
//!   Use `speech()` method.
//! - **Unidirectional WebSocket (v3)**: Simple streaming TTS, text in, audio out.
//!   Use `speech_ws_v3_uni()` method.
//! - **HTTP Streaming (v3)**: HTTP streaming TTS with JSON responses.
//!   Use `speech_http_v3()` method.

mod speech;
mod speech_http_v3;
mod speech_ws_v3_uni;

pub use speech::*;
pub use speech_http_v3::*;
pub use speech_ws_v3_uni::*;

use crate::Client;

/// TTS (Text-to-Speech) API group.
///
/// Provides access to multiple Doubao TTS APIs:
/// - v3 bidirectional WebSocket (`speech()`)
/// - v3 unidirectional WebSocket (`speech_ws_v3_uni()`)
/// - v3 HTTP streaming (`speech_http_v3()`)
pub struct Tts<'c> {
    client: &'c Client,
}

impl<'c> Tts<'c> {
    /// Create a new TTS API group.
    pub(crate) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Get the Speech API (v3 bidirectional WebSocket).
    ///
    /// This is the default TTS API that supports full-duplex streaming
    /// with session management. Use this for advanced use cases that
    /// require bidirectional communication.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// use novel_doubao::Client;
    /// use novel_doubao::spec::tts::CreateSpeechRequestArgs;
    ///
    /// let client = Client::new();
    /// let request = CreateSpeechRequestArgs::default()
    ///     .text("Hello, world!")
    ///     .speaker("zh_female_cancan_mars_bigtts")
    ///     .build()?;
    ///
    /// let response = client.tts().speech().create(request).await?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    #[must_use]
    pub fn speech(&self) -> Speech<'_> {
        Speech::new(self.client)
    }

    /// Get the Speech API (v3 unidirectional WebSocket streaming).
    ///
    /// This API sends text in one request and streams audio back as JSON
    /// chunks with base64-encoded audio data. It's simpler than the
    /// bidirectional API but doesn't support session management or interruption.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// use novel_doubao::Client;
    /// use novel_doubao::spec::tts::CreateSpeechRequestArgs;
    ///
    /// let client = Client::new();
    /// let request = CreateSpeechRequestArgs::default()
    ///     .text("Hello, world!")
    ///     .speaker("zh_female_cancan_mars_bigtts")
    ///     .build()?;
    ///
    /// let response = client.tts().speech_ws_v3_uni().create(request).await?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    #[must_use]
    pub fn speech_ws_v3_uni(&self) -> SpeechWsV3Uni<'_> {
        SpeechWsV3Uni::new(self.client)
    }

    /// Get the Speech API (v3 HTTP streaming).
    ///
    /// This TTS API sends text in a POST request and returns streaming
    /// JSON responses with base64-encoded audio data.
    /// Use this for simple streaming use cases over HTTP.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// use novel_doubao::Client;
    /// use novel_doubao::spec::tts::CreateSpeechRequestArgs;
    ///
    /// let client = Client::new();
    /// let request = CreateSpeechRequestArgs::default()
    ///     .text("Hello, world!")
    ///     .speaker("zh_female_cancan_mars_bigtts")
    ///     .build()?;
    ///
    /// let response = client.tts().speech_http_v3().create(request).await?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    #[must_use]
    pub fn speech_http_v3(&self) -> SpeechHttpV3<'_> {
        SpeechHttpV3::new(self.client)
    }
}
