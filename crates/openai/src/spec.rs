//! Types used in OpenAI API requests and responses.
//! These types are created from component schemas in the [OpenAPI spec](https://github.com/openai/openai-openapi)

// Allow ambiguous glob re-exports since multiple modules legitimately define types with the same
// names (e.g., MessageRole in assistants and responses, Role in chat and responses)
// Users should use explicit module paths for these conflicting types.
#![allow(ambiguous_glob_reexports)]

#[cfg(feature = "administration-types")]
pub mod admin;
#[cfg(feature = "assistant-types")]
pub mod assistants;
#[cfg(feature = "audio-types")]
pub mod audio;
#[cfg(feature = "batch-types")]
pub mod batches;
#[cfg(feature = "chat-completion-types")]
pub mod chat;
#[cfg(feature = "chatkit-types")]
pub mod chatkit;
#[cfg(feature = "completion-types")]
pub mod completions;
#[cfg(feature = "container-types")]
pub mod containers;
#[cfg(feature = "embedding-types")]
pub mod embeddings;
#[cfg(feature = "eval-types")]
pub mod evals;
#[cfg(feature = "file-types")]
pub mod files;
#[cfg(feature = "finetuning-types")]
pub mod finetuning;
#[cfg(feature = "grader-types")]
pub mod graders;
#[cfg(feature = "image-types")]
pub mod images;
#[cfg(any(
    feature = "audio-types",
    feature = "file-types",
    feature = "upload-types",
    feature = "image-types",
    feature = "video-types",
    feature = "container-types",
    feature = "chat-completion-types",
    feature = "realtime-types"
))]
mod input_source;
#[cfg(any(feature = "response-types", feature = "realtime-types"))]
pub mod mcp;
#[cfg(any(
    feature = "response-types",
    feature = "audio-types",
    feature = "video-types",
    feature = "image-types",
    feature = "batch-types",
    feature = "file-types",
    feature = "upload-types",
    feature = "vectorstore-types",
    feature = "container-types",
    feature = "chat-completion-types",
    feature = "realtime-types"
))]
mod metadata;
#[cfg(feature = "model-types")]
pub mod models;
#[cfg(feature = "moderation-types")]
pub mod moderations;
#[cfg_attr(docsrs, doc(cfg(feature = "realtime-types")))]
#[cfg(feature = "realtime-types")]
pub mod realtime;
#[cfg(feature = "response-types")]
pub mod responses;
#[cfg(any(
    feature = "response-types",
    feature = "video-types",
    feature = "vectorstore-types",
    feature = "chat-completion-types",
    feature = "assistant-types",
    feature = "batch-types",
    feature = "audio-types",
    feature = "realtime-types",
    feature = "image-types"
))]
mod shared;
#[cfg(feature = "chat-completion-types")]
mod text;
#[cfg(feature = "upload-types")]
pub mod uploads;
#[cfg(feature = "vectorstore-types")]
pub mod vectorstores;
#[cfg(feature = "video-types")]
pub mod videos;
#[cfg_attr(docsrs, doc(cfg(feature = "webhook-types")))]
#[cfg(feature = "webhook-types")]
pub mod webhooks;

#[cfg(any(
    feature = "audio-types",
    feature = "file-types",
    feature = "upload-types",
    feature = "image-types",
    feature = "video-types",
    feature = "container-types",
    feature = "chat-completion-types",
    feature = "realtime-types"
))]
pub use input_source::*;
#[cfg(any(
    feature = "audio-types",
    feature = "batch-types",
    feature = "file-types",
    feature = "upload-types",
    feature = "image-types",
    feature = "video-types",
    feature = "vectorstore-types",
    feature = "container-types",
    feature = "response-types",
    feature = "chat-completion-types",
    feature = "realtime-types"
))]
pub use metadata::*;

#[cfg(any(
    feature = "audio-types",
    feature = "file-types",
    feature = "image-types",
    feature = "chat-completion-types",
    feature = "completion-types",
    feature = "embedding-types",
    feature = "moderation-types",
    feature = "video-types"
))]
mod impls;

#[cfg(any(
    feature = "response-types",
    feature = "audio-types",
    feature = "file-types",
    feature = "image-types",
    feature = "chat-completion-types",
    feature = "completion-types",
    feature = "embedding-types",
    feature = "moderation-types",
    feature = "administration-types",
))]
impl From<derive_builder::UninitializedFieldError> for crate::error::OpenAIError {
    fn from(value: derive_builder::UninitializedFieldError) -> Self {
        crate::error::OpenAIError::InvalidArgument(value.to_string())
    }
}

// Re-export types from submodules for backward compatibility
#[cfg(feature = "assistant-types")]
pub use assistants::*;
#[cfg(feature = "audio-types")]
pub use audio::*;
#[cfg(feature = "batch-types")]
pub use batches::*;
#[cfg(feature = "chat-completion-types")]
pub use chat::*;
#[cfg(feature = "chatkit-types")]
pub use chatkit::*;
#[cfg(feature = "completion-types")]
pub use completions::*;
#[cfg(feature = "container-types")]
pub use containers::*;
#[cfg(feature = "embedding-types")]
pub use embeddings::*;
#[cfg(feature = "eval-types")]
pub use evals::*;
#[cfg(feature = "file-types")]
pub use files::*;
#[cfg(feature = "finetuning-types")]
pub use finetuning::*;
#[cfg(feature = "grader-types")]
pub use graders::*;
#[cfg(feature = "image-types")]
pub use images::*;
#[cfg(feature = "model-types")]
pub use models::*;
#[cfg(feature = "moderation-types")]
pub use moderations::*;
#[cfg(feature = "response-types")]
pub use responses::*;
#[cfg(any(
    feature = "response-types",
    feature = "video-types",
    feature = "vectorstore-types",
    feature = "chat-completion-types",
    feature = "assistant-types",
    feature = "batch-types",
    feature = "audio-types",
    feature = "realtime-types",
    feature = "image-types"
))]
pub use shared::*;
#[cfg(feature = "chat-completion-types")]
pub use text::*;
#[cfg(feature = "upload-types")]
pub use uploads::*;
#[cfg(feature = "vectorstore-types")]
pub use vectorstores::*;
#[cfg(feature = "video-types")]
pub use videos::*;
#[cfg(feature = "webhook-types")]
pub use webhooks::*;
