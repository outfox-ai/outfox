//! Zhipu AI API client.

#[cfg(feature = "agents")]
use crate::agents::Agents;
#[cfg(feature = "asr")]
use crate::asr::Asr;
#[cfg(feature = "assistant")]
use crate::assistant::Assistant;
#[cfg(feature = "async-task")]
use crate::async_task::AsyncTask;
#[cfg(feature = "batch")]
use crate::batch::Batches;
#[cfg(feature = "chat")]
use crate::chat::Chat;
use crate::config::ZhipuConfig;
#[cfg(feature = "embeddings")]
use crate::embeddings::Embeddings;
#[cfg(feature = "files")]
use crate::files::Files;
#[cfg(feature = "images")]
use crate::images::Images;
#[cfg(feature = "ocr")]
use crate::ocr::Ocr;
#[cfg(feature = "reranking")]
use crate::reranking::Reranking;
#[cfg(feature = "tokenizer")]
use crate::tokenizer::Tokenizer;
#[cfg(feature = "tools")]
use crate::tools::{FileParser, Moderation, WebReader, WebSearch};
#[cfg(feature = "tts")]
use crate::tts::Tts;
#[cfg(feature = "videos")]
use crate::videos::Videos;
#[cfg(feature = "voice")]
use crate::voice::Voice;

/// Zhipu AI API client.
///
/// # Example
///
/// ```no_run
/// use outfox_zhipu::Client;
/// use outfox_zhipu::config::ZhipuConfig;
///
/// // Create a client with default configuration from environment variables.
/// let client = Client::new();
///
/// // Or create with custom configuration.
/// let config = ZhipuConfig::new().with_api_key("your-api-key");
/// let client = Client::with_config(config);
/// ```
#[derive(Clone, Debug)]
pub struct Client {
    config: ZhipuConfig,
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
    /// - `ZHIPUAI_API_KEY` or `ZHIPU_API_KEY`: API key
    /// - `ZHIPUAI_BASE_URL` or `ZHIPU_API_BASE`: API base URL
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: ZhipuConfig::default(),
            http_client: reqwest::Client::new(),
        }
    }

    /// Create a new client with the given configuration.
    #[must_use]
    pub fn with_config(config: ZhipuConfig) -> Self {
        Self {
            config,
            http_client: reqwest::Client::new(),
        }
    }

    /// Create a new client with custom HTTP client.
    #[must_use]
    pub fn with_http_client(mut self, http_client: reqwest::Client) -> Self {
        self.http_client = http_client;
        self
    }

    /// Get the client configuration.
    #[must_use]
    pub fn config(&self) -> &ZhipuConfig {
        &self.config
    }

    /// Get the HTTP client.
    #[must_use]
    pub fn http_client(&self) -> &reqwest::Client {
        &self.http_client
    }

    /// Get the Agents API group.
    #[cfg(feature = "agents")]
    #[cfg_attr(docsrs, doc(cfg(feature = "agents")))]
    #[must_use]
    pub fn agents(&self) -> Agents<'_> {
        Agents::new(self)
    }

    /// Get the Assistant API group.
    #[cfg(feature = "assistant")]
    #[cfg_attr(docsrs, doc(cfg(feature = "assistant")))]
    #[must_use]
    pub fn assistant(&self) -> Assistant<'_> {
        Assistant::new(self)
    }

    /// Get the Batch API group.
    #[cfg(feature = "batch")]
    #[cfg_attr(docsrs, doc(cfg(feature = "batch")))]
    #[must_use]
    pub fn batches(&self) -> Batches<'_> {
        Batches::new(self)
    }

    /// Get the Chat API group.
    #[cfg(feature = "chat")]
    #[must_use]
    pub fn chat(&self) -> Chat<'_> {
        Chat::new(self)
    }

    /// Get the Embeddings API group.
    #[cfg(feature = "embeddings")]
    #[must_use]
    pub fn embeddings(&self) -> Embeddings<'_> {
        Embeddings::new(self)
    }

    /// Get the Files API group.
    #[cfg(feature = "files")]
    #[cfg_attr(docsrs, doc(cfg(feature = "files")))]
    #[must_use]
    pub fn files(&self) -> Files<'_> {
        Files::new(self)
    }

    /// Get the Images API group.
    #[cfg(feature = "images")]
    #[cfg_attr(docsrs, doc(cfg(feature = "images")))]
    #[must_use]
    pub fn images(&self) -> Images<'_> {
        Images::new(self)
    }

    /// Get the OCR API group.
    #[cfg(feature = "ocr")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ocr")))]
    #[must_use]
    pub fn ocr(&self) -> Ocr<'_> {
        Ocr::new(self)
    }

    /// Get the TTS (Text-to-Speech) API group.
    #[cfg(feature = "tts")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tts")))]
    #[must_use]
    pub fn tts(&self) -> Tts<'_> {
        Tts::new(self)
    }

    /// Get the ASR (Speech-to-Text) API group.
    #[cfg(feature = "asr")]
    #[cfg_attr(docsrs, doc(cfg(feature = "asr")))]
    #[must_use]
    pub fn asr(&self) -> Asr<'_> {
        Asr::new(self)
    }

    /// Get the Async Task API group.
    #[cfg(feature = "async-task")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async-task")))]
    #[must_use]
    pub fn async_task(&self) -> AsyncTask<'_> {
        AsyncTask::new(self)
    }

    /// Get the Voice API group.
    #[cfg(feature = "voice")]
    #[cfg_attr(docsrs, doc(cfg(feature = "voice")))]
    #[must_use]
    pub fn voice(&self) -> Voice<'_> {
        Voice::new(self)
    }

    /// Get the Reranking API group.
    #[cfg(feature = "reranking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "reranking")))]
    #[must_use]
    pub fn reranking(&self) -> Reranking<'_> {
        Reranking::new(self)
    }

    /// Get the Tokenizer API group.
    #[cfg(feature = "tokenizer")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokenizer")))]
    #[must_use]
    pub fn tokenizer(&self) -> Tokenizer<'_> {
        Tokenizer::new(self)
    }

    /// Get the Videos API group.
    #[cfg(feature = "videos")]
    #[cfg_attr(docsrs, doc(cfg(feature = "videos")))]
    #[must_use]
    pub fn videos(&self) -> Videos<'_> {
        Videos::new(self)
    }

    /// Get the Web Search API.
    #[cfg(feature = "tools")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tools")))]
    #[must_use]
    pub fn web_search(&self) -> WebSearch<'_> {
        WebSearch::new(self)
    }

    /// Get the Web Reader API.
    #[cfg(feature = "tools")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tools")))]
    #[must_use]
    pub fn web_reader(&self) -> WebReader<'_> {
        WebReader::new(self)
    }

    /// Get the Moderation API.
    #[cfg(feature = "tools")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tools")))]
    #[must_use]
    pub fn moderation(&self) -> Moderation<'_> {
        Moderation::new(self)
    }

    /// Get the File Parser API.
    #[cfg(feature = "tools")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tools")))]
    #[must_use]
    pub fn file_parser(&self) -> FileParser<'_> {
        FileParser::new(self)
    }
}
