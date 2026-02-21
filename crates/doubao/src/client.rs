//! Doubao API client.

#[cfg(feature = "asr")]
use crate::asr::Asr;
#[cfg(feature = "chat")]
use crate::chat::Chat;
use crate::config::DoubaoConfig;
#[cfg(feature = "embeddings")]
use crate::embeddings::Embeddings;
#[cfg(feature = "images")]
use crate::images::Images;
#[cfg(feature = "tokenization")]
use crate::tokenization::Tokenization;
#[cfg(feature = "tts")]
use crate::tts::Tts;

/// Doubao API client.
///
/// # Example
///
/// ```no_run
/// use novel_doubao::Client;
/// use novel_doubao::config::DoubaoConfig;
///
/// // Create a client with default configuration from environment variables.
/// let client = Client::new();
///
/// // Or create with custom configuration.
/// let config = DoubaoConfig::new()
///     .with_app_id("your-app-id")
///     .with_api_key("your-api-key")
///     .with_access_token("your-access-token");
/// let client = Client::with_config(config);
/// ```
#[derive(Clone, Debug)]
pub struct Client {
    config: DoubaoConfig,
    #[cfg(feature = "http")]
    http_client: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    /// Create a new client with default configuration from environment variables.
    ///
    /// Environment variables:
    /// - `DOUBAO_APP_ID`: Application ID
    /// - `DOUBAO_API_KEY`: API key
    /// - `DOUBAO_ACCESS_TOKEN`: Access token
    /// - `DOUBAO_RESOURCE_ID`: Resource ID (default: "seed-tts-2.0")
    /// - `DOUBAO_HTTP_BASE`: HTTP base URL (default: "https://ark.cn-beijing.volces.com/api/v3")
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: DoubaoConfig::default(),
            #[cfg(feature = "http")]
            http_client: reqwest::Client::new(),
        }
    }

    /// Create a new client with the given configuration.
    #[must_use]
    pub fn with_config(config: DoubaoConfig) -> Self {
        Self {
            config,
            #[cfg(feature = "http")]
            http_client: reqwest::Client::new(),
        }
    }

    /// Create a new client with custom HTTP client.
    #[cfg(feature = "http")]
    #[must_use]
    pub fn with_http_client(mut self, http_client: reqwest::Client) -> Self {
        self.http_client = http_client;
        self
    }

    /// Get the client configuration.
    #[must_use]
    pub fn config(&self) -> &DoubaoConfig {
        &self.config
    }

    /// Get the HTTP client.
    #[cfg(feature = "http")]
    #[must_use]
    pub fn http_client(&self) -> &reqwest::Client {
        &self.http_client
    }

    /// Get the TTS API group.
    #[cfg(feature = "tts")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tts")))]
    #[must_use]
    pub fn tts(&self) -> Tts<'_> {
        Tts::new(self)
    }

    /// Get the ASR API group.
    #[cfg(feature = "asr")]
    #[cfg_attr(docsrs, doc(cfg(feature = "asr")))]
    #[must_use]
    pub fn asr(&self) -> Asr<'_> {
        Asr::new(self)
    }

    /// Get the Chat API group.
    #[cfg(feature = "chat")]
    #[cfg_attr(docsrs, doc(cfg(feature = "chat")))]
    #[must_use]
    pub fn chat(&self) -> Chat<'_> {
        Chat::new(self)
    }

    /// Get the Embeddings API group.
    #[cfg(feature = "embeddings")]
    #[cfg_attr(docsrs, doc(cfg(feature = "embeddings")))]
    #[must_use]
    pub fn embeddings(&self) -> Embeddings<'_> {
        Embeddings::new(self)
    }

    /// Get the Images API group.
    #[cfg(feature = "images")]
    #[cfg_attr(docsrs, doc(cfg(feature = "images")))]
    #[must_use]
    pub fn images(&self) -> Images<'_> {
        Images::new(self)
    }

    /// Get the Tokenization API group.
    #[cfg(feature = "tokenization")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokenization")))]
    #[must_use]
    pub fn tokenization(&self) -> Tokenization<'_> {
        Tokenization::new(self)
    }
}
