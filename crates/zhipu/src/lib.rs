//! Rust library for Zhipu AI (GLM) APIs.
//!
//! ## Creating a client
//!
//! ```no_run
//! use outfox_zhipu::Client;
//! use outfox_zhipu::config::ZhipuConfig;
//!
//! // Create a client with default configuration from environment variables.
//! let client = Client::new();
//!
//! // Or create with custom configuration.
//! let config = ZhipuConfig::new().with_api_key("your-api-key");
//! let client = Client::with_config(config);
//! ```
//!
//! ## Chat Completions
//!
//! ```no_run
//! # tokio_test::block_on(async {
//! use outfox_zhipu::Client;
//! use outfox_zhipu::spec::chat::{ChatMessage, CreateChatCompletionRequestArgs, Model};
//!
//! let client = Client::new();
//!
//! let request = CreateChatCompletionRequestArgs::default()
//!     .model(Model::Glm4)
//!     .messages(vec![
//!         ChatMessage::system("You are a helpful assistant."),
//!         ChatMessage::user("Hello!"),
//!     ])
//!     .build()?;
//!
//! let response = client.chat().create(request).await?;
//! println!("{}", response.choices[0].message.content);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! # });
//! ```
//!
//! ## Streaming Chat Completions
//!
//! ```no_run
//! # tokio_test::block_on(async {
//! use futures_util::StreamExt;
//! use outfox_zhipu::Client;
//! use outfox_zhipu::spec::chat::{ChatMessage, CreateChatCompletionRequestArgs};
//!
//! let client = Client::new();
//!
//! let request = CreateChatCompletionRequestArgs::default()
//!     .model("glm-4")
//!     .messages(vec![ChatMessage::user("Tell me a story.")])
//!     .build()?;
//!
//! let mut stream = client.chat().create_stream(request).await?;
//! while let Some(chunk) = stream.next().await {
//!     if let Ok(chunk) = chunk {
//!         if let Some(content) = &chunk.choices[0].delta.content {
//!             print!("{}", content);
//!         }
//!     }
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! # });
//! ```
//!
//! ## Embeddings
//!
//! ```no_run
//! # tokio_test::block_on(async {
//! use outfox_zhipu::Client;
//!
//! let client = Client::new();
//!
//! let embedding = client
//!     .embeddings()
//!     .embed_text("embedding-2", "Hello, world!")
//!     .await?;
//!
//! println!("Embedding dimension: {}", embedding.len());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! # });
//! ```
//!
//! ## Image Generation
//!
//! ```no_run
//! # tokio_test::block_on(async {
//! use outfox_zhipu::Client;
//!
//! let client = Client::new();
//!
//! let image = client
//!     .images()
//!     .generate("cogview-3", "A beautiful sunset over the ocean")
//!     .await?;
//!
//! image.save("sunset.png").await?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! # });
//! ```
//!
//! ## Environment Variables
//!
//! - `ZHIPUAI_API_KEY` or `ZHIPU_API_KEY`: API key
//! - `ZHIPUAI_BASE_URL` or `ZHIPU_API_BASE`: API base URL (default: https://open.bigmodel.cn/api/paas/v4)
//!
//! ## Text-to-Speech
//!
//! ```no_run
//! # tokio_test::block_on(async {
//! use outfox_zhipu::Client;
//!
//! let client = Client::new();
//!
//! // Simple synthesis
//! let audio = client.tts().synthesize("你好，今天天气怎么样？").await?;
//!
//! audio.save("output.wav").await?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! # });
//! ```
//!
//! ## Speech-to-Text
//!
//! ```no_run
//! # tokio_test::block_on(async {
//! use outfox_zhipu::Client;
//!
//! let client = Client::new();
//!
//! let result = client.asr().transcribe_file("audio.wav").await?;
//!
//! println!("Transcription: {}", result.text);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! # });
//! ```
//!
//! ## Environment Variables
//!
//! - `ZHIPUAI_API_KEY` or `ZHIPU_API_KEY`: API key
//! - `ZHIPUAI_BASE_URL` or `ZHIPU_API_BASE`: API base URL (default: https://open.bigmodel.cn/api/paas/v4)
//!
//! ## Features
//!
//! - `chat`: Enable Chat Completions API
//! - `embeddings`: Enable Embeddings API
//! - `images`: Enable Images API
//! - `tts`: Enable Text-to-Speech API
//! - `asr`: Enable Speech-to-Text API
//! - `async-task`: Enable Async Task APIs (async chat, video, image generation)
//! - `voice`: Enable Voice APIs (clone, list, delete)
//! - `reranking`: Enable Text Reranking API
//! - `tokenizer`: Enable Text Tokenizer API
//! - `tools`: Enable Tool APIs (web search, web reader, moderation, file parser)
//! - `full`: Enable all features
//! - `rustls`: Use rustls for TLS (default)
//! - `native-tls`: Use native-tls for TLS
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "agents")]
mod agents;
#[cfg(feature = "asr")]
mod asr;
#[cfg(feature = "assistant")]
mod assistant;
#[cfg(feature = "async-task")]
mod async_task;
#[cfg(feature = "batch")]
mod batch;
#[cfg(feature = "chat")]
mod chat;
mod client;
pub mod config;
#[cfg(feature = "embeddings")]
mod embeddings;
pub mod error;
#[cfg(feature = "files")]
mod files;
#[cfg(feature = "images")]
mod images;
#[cfg(feature = "ocr")]
mod ocr;
#[cfg(feature = "reranking")]
mod reranking;
pub mod spec;
#[cfg(feature = "tokenizer")]
mod tokenizer;
#[cfg(feature = "tools")]
mod tools;
#[cfg(feature = "tts")]
mod tts;
#[cfg(feature = "videos")]
mod videos;
#[cfg(feature = "voice")]
mod voice;

#[cfg(feature = "agents")]
#[cfg_attr(docsrs, doc(cfg(feature = "agents")))]
pub use agents::Agents;
#[cfg(feature = "asr")]
#[cfg_attr(docsrs, doc(cfg(feature = "asr")))]
pub use asr::Asr;
#[cfg(feature = "assistant")]
#[cfg_attr(docsrs, doc(cfg(feature = "assistant")))]
pub use assistant::Assistant;
#[cfg(feature = "async-task")]
#[cfg_attr(docsrs, doc(cfg(feature = "async-task")))]
pub use async_task::AsyncTask;
#[cfg(feature = "batch")]
#[cfg_attr(docsrs, doc(cfg(feature = "batch")))]
pub use batch::Batches;
#[cfg(feature = "chat")]
#[cfg_attr(docsrs, doc(cfg(feature = "chat")))]
pub use chat::Chat;
pub use client::Client;
#[cfg(feature = "embeddings")]
#[cfg_attr(docsrs, doc(cfg(feature = "embeddings")))]
pub use embeddings::Embeddings;
#[cfg(feature = "files")]
#[cfg_attr(docsrs, doc(cfg(feature = "files")))]
pub use files::Files;
#[cfg(feature = "images")]
#[cfg_attr(docsrs, doc(cfg(feature = "images")))]
pub use images::Images;
#[cfg(feature = "ocr")]
#[cfg_attr(docsrs, doc(cfg(feature = "ocr")))]
pub use ocr::Ocr;
#[cfg(feature = "reranking")]
#[cfg_attr(docsrs, doc(cfg(feature = "reranking")))]
pub use reranking::Reranking;
#[cfg(feature = "tokenizer")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokenizer")))]
pub use tokenizer::Tokenizer;
#[cfg(feature = "tools")]
#[cfg_attr(docsrs, doc(cfg(feature = "tools")))]
pub use tools::{FileParser, Moderation, WebReader, WebSearch};
#[cfg(feature = "tts")]
#[cfg_attr(docsrs, doc(cfg(feature = "tts")))]
pub use tts::Tts;
#[cfg(feature = "videos")]
#[cfg_attr(docsrs, doc(cfg(feature = "videos")))]
pub use videos::Videos;
#[cfg(feature = "voice")]
#[cfg_attr(docsrs, doc(cfg(feature = "voice")))]
pub use voice::Voice;
