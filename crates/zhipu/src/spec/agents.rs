//! Agents API request and response types.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::spec::videos::SensitiveWordCheck;

/// Agents invoke request.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(name = "InvokeAgentRequestArgs", setter(into, strip_option))]
pub struct InvokeAgentRequest {
    /// Agent ID.
    pub agent_id: String,
    /// Messages to send.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<AgentMessages>,
    /// Whether to stream the response.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Request ID.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// User ID.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Custom variables.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_variables: Option<serde_json::Value>,
    /// Sensitive word check configuration.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive_word_check: Option<SensitiveWordCheck>,
}

/// Agent messages input.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AgentMessages {
    /// Single text message.
    Text(String),
    /// List of text messages.
    TextList(Vec<String>),
    /// Token IDs.
    Tokens(Vec<i32>),
    /// Custom message object.
    Object(serde_json::Value),
}

impl From<&str> for AgentMessages {
    fn from(s: &str) -> Self {
        AgentMessages::Text(s.to_string())
    }
}

impl From<String> for AgentMessages {
    fn from(s: String) -> Self {
        AgentMessages::Text(s)
    }
}

/// Agent async result request.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(name = "AgentAsyncResultRequestArgs", setter(into, strip_option))]
pub struct AgentAsyncResultRequest {
    /// Agent ID.
    pub agent_id: String,
    /// Async task ID.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub async_id: Option<String>,
    /// Conversation ID.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    /// Custom variables.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_variables: Option<serde_json::Value>,
}

/// Agent message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    /// Role of the message sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Content of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// Agent choice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentChoice {
    /// Index of the choice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// Message content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<AgentMessage>,
    /// Delta content (for streaming).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<AgentMessage>,
    /// Finish reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
}

/// Agent usage.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentUsage {
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

/// Agent completion response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCompletion {
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
    pub choices: Vec<AgentChoice>,
    /// Token usage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<AgentUsage>,
    /// Conversation ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    /// Task status (for async).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status: Option<String>,
}

/// Agent completion chunk (for streaming).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCompletionChunk {
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
    pub choices: Vec<AgentChoice>,
    /// Token usage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<AgentUsage>,
}
