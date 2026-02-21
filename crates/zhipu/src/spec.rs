//! Type definitions for Zhipu AI APIs.

#[cfg(feature = "agents-types")]
#[cfg_attr(docsrs, doc(cfg(feature = "agents-types")))]
pub mod agents;

#[cfg(feature = "asr-types")]
#[cfg_attr(docsrs, doc(cfg(feature = "asr-types")))]
pub mod asr;

#[cfg(feature = "assistant-types")]
#[cfg_attr(docsrs, doc(cfg(feature = "assistant-types")))]
pub mod assistant;

#[cfg(feature = "async-task-types")]
#[cfg_attr(docsrs, doc(cfg(feature = "async-task-types")))]
pub mod async_task;

#[cfg(feature = "batch-types")]
#[cfg_attr(docsrs, doc(cfg(feature = "batch-types")))]
pub mod batch;

#[cfg(feature = "chat-types")]
#[cfg_attr(docsrs, doc(cfg(feature = "chat-types")))]
pub mod chat;

#[cfg(feature = "embeddings-types")]
#[cfg_attr(docsrs, doc(cfg(feature = "embeddings-types")))]
pub mod embeddings;

#[cfg(feature = "files-types")]
#[cfg_attr(docsrs, doc(cfg(feature = "files-types")))]
pub mod files;

#[cfg(feature = "images-types")]
#[cfg_attr(docsrs, doc(cfg(feature = "images-types")))]
pub mod images;

#[cfg(feature = "ocr-types")]
#[cfg_attr(docsrs, doc(cfg(feature = "ocr-types")))]
pub mod ocr;

#[cfg(feature = "reranking-types")]
#[cfg_attr(docsrs, doc(cfg(feature = "reranking-types")))]
pub mod reranking;

#[cfg(feature = "tokenizer-types")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokenizer-types")))]
pub mod tokenizer;

#[cfg(feature = "tools-types")]
#[cfg_attr(docsrs, doc(cfg(feature = "tools-types")))]
pub mod tools;

#[cfg(feature = "tts-types")]
#[cfg_attr(docsrs, doc(cfg(feature = "tts-types")))]
pub mod tts;

#[cfg(feature = "videos-types")]
#[cfg_attr(docsrs, doc(cfg(feature = "videos-types")))]
pub mod videos;

#[cfg(feature = "voice-types")]
#[cfg_attr(docsrs, doc(cfg(feature = "voice-types")))]
pub mod voice;
