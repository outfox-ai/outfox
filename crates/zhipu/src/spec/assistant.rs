//! Assistant API request and response types.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Assistant message role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum AssistantMessageRole {
    /// User message.
    #[default]
    User,
    /// Assistant message.
    Assistant,
}

/// Assistant conversation message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMessage {
    /// Role of the message sender.
    pub role: AssistantMessageRole,
    /// Content of the message.
    pub content: String,
}

impl ConversationMessage {
    /// Create a user message.
    #[must_use]
    pub fn user<S: Into<String>>(content: S) -> Self {
        Self {
            role: AssistantMessageRole::User,
            content: content.into(),
        }
    }

    /// Create an assistant message.
    #[must_use]
    pub fn assistant<S: Into<String>>(content: S) -> Self {
        Self {
            role: AssistantMessageRole::Assistant,
            content: content.into(),
        }
    }
}

/// Assistant attachment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantAttachment {
    /// Attachment type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub attachment_type: Option<String>,
    /// File ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Extra parameters for assistant.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssistantExtraParameters {
    /// Temperature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Top P.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// Max tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
}

/// Assistant conversation request.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(name = "AssistantConversationRequestArgs", setter(into, strip_option))]
pub struct AssistantConversationRequest {
    /// Assistant ID.
    pub assistant_id: String,
    /// Messages.
    pub messages: Vec<ConversationMessage>,
    /// Model (optional).
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Whether to stream.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Conversation ID.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    /// Attachments.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AssistantAttachment>>,
    /// Metadata.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /// Request ID.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// User ID.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Extra parameters.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_parameters: Option<AssistantExtraParameters>,
}

/// Assistant completion message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantCompletionMessage {
    /// Role of the message sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Content of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// Assistant completion choice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantCompletionChoice {
    /// Index.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// Message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<AssistantCompletionMessage>,
    /// Delta (for streaming).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<AssistantCompletionMessage>,
    /// Finish reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
}

/// Assistant completion usage.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssistantCompletionUsage {
    /// Prompt tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_tokens: Option<i64>,
    /// Completion tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens: Option<i64>,
    /// Total tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tokens: Option<i64>,
}

/// Assistant completion response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantCompletion {
    /// Response ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Object type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    /// Creation timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    /// Model used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Choices.
    #[serde(default)]
    pub choices: Vec<AssistantCompletionChoice>,
    /// Usage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<AssistantCompletionUsage>,
    /// Conversation ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
}

/// Query assistant support request.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(name = "QueryAssistantSupportRequestArgs", setter(into, strip_option))]
pub struct QueryAssistantSupportRequest {
    /// List of assistant IDs.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant_id_list: Option<Vec<String>>,
    /// Request ID.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// User ID.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// Assistant support info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantSupportInfo {
    /// Assistant ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant_id: Option<String>,
    /// Assistant name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported: Option<bool>,
}

/// Query assistant support response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryAssistantSupportResponse {
    /// List of assistant support info.
    #[serde(default)]
    pub data: Vec<AssistantSupportInfo>,
}

/// Query conversation usage request.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(name = "QueryConversationUsageRequestArgs", setter(into, strip_option))]
pub struct QueryConversationUsageRequest {
    /// Assistant ID.
    pub assistant_id: String,
    /// Page number.
    #[builder(default = "Some(1)")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    /// Page size.
    #[builder(default = "Some(10)")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// Request ID.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// User ID.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// Conversation usage item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationUsageItem {
    /// Conversation ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    /// Creation time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// Update time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
    /// Message count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_count: Option<i32>,
}

/// Query conversation usage response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryConversationUsageResponse {
    /// List of conversation usage items.
    #[serde(default)]
    pub data: Vec<ConversationUsageItem>,
    /// Total count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// Page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}
