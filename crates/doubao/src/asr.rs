//! ASR API group.

mod recognition;
mod streaming;
pub use recognition::*;
pub use streaming::*;

use crate::Client;

/// ASR (Automatic Speech Recognition) API group.
///
/// Provides access to the Doubao ASR v3 APIs:
/// - Standard file recognition (submit + query)
/// - Flash/turbo file recognition (single request)
/// - Streaming recognition (WebSocket)
pub struct Asr<'c> {
    client: &'c Client,
}

impl<'c> Asr<'c> {
    /// Create a new ASR API group.
    pub(crate) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Get the Recognition API for file-based recognition.
    #[must_use]
    pub fn recognition(&self) -> Recognition<'_> {
        Recognition::new(self.client)
    }

    /// Get the Streaming API for real-time recognition.
    #[must_use]
    pub fn streaming(&self) -> Streaming<'_> {
        Streaming::new(self.client)
    }
}
