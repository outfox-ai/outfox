//! Rust library for Doubao (ByteDance Volcengine) APIs.
//!
//! This crate provides clients for various Doubao APIs:
//!
//! - **TTS**: Text-to-Speech API
//! - **ASR**: Automatic Speech Recognition API
//! - **Chat**: Chat completion API (OpenAI-compatible)
//! - **Embeddings**: Text and multimodal embeddings API
//! - **Images**: Image generation API
//! - **Tokenization**: Text tokenization API
//!
//! ## Creating a client
//!
//! ```no_run
//! use outfox_doubao::Client;
//! use outfox_doubao::config::DoubaoConfig;
//!
//! // Create a client with default configuration from environment variables.
//! let client = Client::new();
//!
//! // Or create with custom configuration.
//! let config = DoubaoConfig::new()
//!     .with_app_id("your-app-id")
//!     .with_api_key("your-api-key")
//!     .with_access_token("your-access-token")
//!     .with_resource_id("seed-tts-2.0");
//! let client = Client::with_config(config);
//! ```
//!
//! ## Making TTS requests
//!
//! ```no_run
//! # tokio_test::block_on(async {
//! use outfox_doubao::Client;
//! use outfox_doubao::spec::tts::CreateSpeechRequestArgs;
//!
//! let client = Client::new();
//!
//! let request = CreateSpeechRequestArgs::default()
//!     .text("Hello, world!")
//!     .speaker("zh_female_cancan_mars_bigtts")
//!     .sample_rate(24000u32)
//!     .build()?;
//!
//! let response = client.tts().speech().create(request).await?;
//!
//! // Save to file
//! response.save("output.mp3").await?;
//!
//! println!("Generated {} bytes of audio", response.bytes.len());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! # });
//! ```
//!
//! ## Making ASR requests
//!
//! ### Flash recognition (fastest, single request)
//!
//! ```no_run
//! # tokio_test::block_on(async {
//! use outfox_doubao::Client;
//!
//! let client = Client::new();
//!
//! // Recognize from URL
//! let result = client
//!     .asr()
//!     .recognition()
//!     .flash_url("https://example.com/audio.wav", "user-id")
//!     .await?;
//!
//! println!("Recognized: {}", result.result.text);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! # });
//! ```
//!
//! ### Standard recognition (for long audio files)
//!
//! ```no_run
//! # tokio_test::block_on(async {
//! use outfox_doubao::Client;
//!
//! let client = Client::new();
//!
//! // Submit and wait for result
//! let result = client
//!     .asr()
//!     .recognition()
//!     .recognize_url("https://example.com/audio.wav", "user-id")
//!     .await?;
//!
//! println!("Recognized: {}", result.result.text);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! # });
//! ```
//!
//! ### Streaming recognition (real-time)
//!
//! ```no_run
//! # tokio_test::block_on(async {
//! use bytes::Bytes;
//! use outfox_doubao::Client;
//! use outfox_doubao::spec::asr::StreamingAsrConfigArgs;
//!
//! let client = Client::new();
//!
//! let config = StreamingAsrConfigArgs::default().rate(16000u32).build()?;
//!
//! let mut session = client.asr().streaming().create_session(config).await?;
//!
//! // Send audio data
//! session
//!     .send_audio(Bytes::from_static(b"audio data..."))
//!     .await?;
//!
//! // Receive results
//! while let Some(result) = session.recv().await {
//!     println!(
//!         "Partial: {} (final: {})",
//!         result.result.text, result.is_final
//!     );
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! # });
//! ```
//!
//! ## Environment Variables
//!
//! The client reads these environment variables by default:
//! - `DOUBAO_APP_ID`: Application ID
//! - `DOUBAO_API_KEY`: API key
//! - `DOUBAO_ACCESS_TOKEN`: Access token
//! - `DOUBAO_RESOURCE_ID`: Resource ID (default: "seed-tts-2.0")
//! - `DOUBAO_HTTP_BASE`: HTTP base URL (default: "https://ark.cn-beijing.volces.com/api/v3")
//!
//! ## Features
//!
//! - `tts`: Enable TTS (Text-to-Speech) API
//! - `asr`: Enable ASR (Automatic Speech Recognition) API
//! - `chat`: Enable Chat completion API
//! - `embeddings`: Enable Embeddings API
//! - `images`: Enable Image generation API
//! - `tokenization`: Enable Tokenization API
//! - `full`: Enable all features
//! - `rustls`: Use rustls for TLS (default)
//! - `native-tls`: Use native-tls for TLS
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "asr")]
mod asr;
#[cfg(feature = "chat")]
mod chat;
mod client;
pub mod config;
#[cfg(feature = "embeddings")]
mod embeddings;
pub mod error;
#[cfg(feature = "images")]
mod images;
pub mod spec;
#[cfg(feature = "tokenization")]
mod tokenization;
#[cfg(feature = "tts")]
mod tts;

#[cfg(feature = "asr")]
#[cfg_attr(docsrs, doc(cfg(feature = "asr")))]
pub use asr::{Asr, Recognition, Streaming, StreamingSession};
#[cfg(feature = "chat")]
#[cfg_attr(docsrs, doc(cfg(feature = "chat")))]
pub use chat::Chat;
pub use client::Client;
#[cfg(feature = "embeddings")]
#[cfg_attr(docsrs, doc(cfg(feature = "embeddings")))]
pub use embeddings::Embeddings;
#[cfg(feature = "images")]
#[cfg_attr(docsrs, doc(cfg(feature = "images")))]
pub use images::Images;
#[cfg(feature = "tokenization")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokenization")))]
pub use tokenization::Tokenization;
#[cfg(feature = "tts")]
#[cfg_attr(docsrs, doc(cfg(feature = "tts")))]
pub use tts::{Speech, Tts};
