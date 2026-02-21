//! Type definitions for Doubao APIs.

#[cfg(feature = "asr-types")]
pub mod asr;

#[cfg(feature = "chat-types")]
pub mod chat;

#[cfg(feature = "embeddings-types")]
pub mod embeddings;

#[cfg(feature = "images-types")]
pub mod images;

#[cfg(feature = "tokenization-types")]
pub mod tokenization;

#[cfg(feature = "tts-types")]
pub mod tts;
