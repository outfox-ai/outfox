//! Chat completion request and response types.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::ZhipuError;

/// Role in a chat conversation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// System message for setting behavior.
    System,
    /// User message.
    #[default]
    User,
    /// Assistant response.
    Assistant,
    /// Tool/function response.
    Tool,
}

/// A message in the chat conversation.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChatMessage {
    /// The role of the message author.
    pub role: Role,
    /// The content of the message.
    pub content: String,
    /// Optional name for the participant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Tool call ID (for tool responses).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    /// Tool calls made by the assistant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

impl ChatMessage {
    /// Create a new system message.
    #[must_use]
    pub fn system<S: Into<String>>(content: S) -> Self {
        Self {
            role: Role::System,
            content: content.into(),
            ..Default::default()
        }
    }

    /// Create a new user message.
    #[must_use]
    pub fn user<S: Into<String>>(content: S) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
            ..Default::default()
        }
    }

    /// Create a new assistant message.
    #[must_use]
    pub fn assistant<S: Into<String>>(content: S) -> Self {
        Self {
            role: Role::Assistant,
            content: content.into(),
            ..Default::default()
        }
    }

    /// Create a new tool response message.
    #[must_use]
    pub fn tool<S: Into<String>>(tool_call_id: S, content: S) -> Self {
        Self {
            role: Role::Tool,
            content: content.into(),
            tool_call_id: Some(tool_call_id.into()),
            ..Default::default()
        }
    }
}

/// A tool call made by the assistant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// The ID of the tool call.
    pub id: String,
    /// The type of tool call (always "function").
    #[serde(rename = "type")]
    pub kind: String,
    /// The function to call.
    pub function: FunctionCall,
}

/// A function call within a tool call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    /// The name of the function.
    pub name: String,
    /// The arguments to the function (JSON string).
    pub arguments: String,
}

/// Tool definition for function calling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// The type of tool (always "function").
    #[serde(rename = "type")]
    pub kind: String,
    /// The function definition.
    pub function: FunctionDefinition,
}

impl Tool {
    /// Create a new function tool.
    #[must_use]
    pub fn function(name: &str, description: &str, parameters: serde_json::Value) -> Self {
        Self {
            kind: "function".to_string(),
            function: FunctionDefinition {
                name: name.to_string(),
                description: Some(description.to_string()),
                parameters: Some(parameters),
            },
        }
    }
}

/// Function definition for tool calling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDefinition {
    /// The name of the function.
    pub name: String,
    /// Description of what the function does.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The parameters the function accepts (JSON Schema).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}

/// Tool choice configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolChoice {
    /// Let the model decide.
    Auto,
    /// Don't use tools.
    None,
    /// Force using tools.
    Required,
    /// Use a specific function.
    Function { name: String },
}

impl Default for ToolChoice {
    fn default() -> Self {
        Self::Auto
    }
}

/// Response format configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseFormat {
    /// The type of response format.
    #[serde(rename = "type")]
    pub kind: String,
}

impl ResponseFormat {
    /// Text response format.
    #[must_use]
    pub fn text() -> Self {
        Self {
            kind: "text".to_string(),
        }
    }

    /// JSON object response format.
    #[must_use]
    pub fn json_object() -> Self {
        Self {
            kind: "json_object".to_string(),
        }
    }
}

/// Request to create a chat completion.
#[derive(Clone, Default, Debug, Builder, Serialize, Deserialize)]
#[builder(name = "CreateChatCompletionRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "ZhipuError"))]
pub struct CreateChatCompletionRequest {
    /// ID of the model to use.
    pub model: String,

    /// The messages to generate chat completions for.
    pub messages: Vec<ChatMessage>,

    /// Whether to stream the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Sampling temperature (0-2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Nucleus sampling parameter (0-1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// Maximum tokens to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    /// Stop sequences (up to 4).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,

    /// Presence penalty (-2 to 2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,

    /// Frequency penalty (-2 to 2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,

    /// Number of completions to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,

    /// Tools available to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,

    /// Tool choice configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<String>,

    /// Response format configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,

    /// User identifier for tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    /// Request ID for tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,

    /// Enable web search tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_sample: Option<bool>,
}

/// Token usage statistics.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Usage {
    /// Number of tokens in the prompt.
    pub prompt_tokens: u32,
    /// Number of tokens in the completion.
    pub completion_tokens: u32,
    /// Total number of tokens.
    pub total_tokens: u32,
}

/// A choice in the chat completion response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChoice {
    /// The index of this choice.
    pub index: u32,
    /// The generated message.
    pub message: ChatMessage,
    /// The reason the model stopped generating.
    pub finish_reason: Option<String>,
}

/// Response from the chat completion API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChatCompletionResponse {
    /// Unique identifier for the completion.
    pub id: String,
    /// Object type (always "chat.completion").
    pub object: String,
    /// Unix timestamp of creation.
    pub created: u64,
    /// Model used for the completion.
    pub model: String,
    /// List of completion choices.
    pub choices: Vec<ChatChoice>,
    /// Token usage statistics.
    #[serde(default)]
    pub usage: Usage,
}

/// A delta in a streaming response.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChatDelta {
    /// The role of the message (only in first chunk).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    /// Content fragment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Tool calls being made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCallDelta>>,
}

/// A tool call delta in streaming.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallDelta {
    /// Index of the tool call.
    pub index: u32,
    /// The ID of the tool call (only in first chunk).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The type of tool call.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The function being called.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<FunctionCallDelta>,
}

/// A function call delta in streaming.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FunctionCallDelta {
    /// The name of the function (only in first chunk).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Arguments fragment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
}

/// A choice in a streaming response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChoiceDelta {
    /// The index of this choice.
    pub index: u32,
    /// The delta content.
    pub delta: ChatDelta,
    /// The reason the model stopped generating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
}

/// A chunk in the streaming response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionChunk {
    /// Unique identifier for the completion.
    pub id: String,
    /// Object type (always "chat.completion.chunk").
    pub object: String,
    /// Unix timestamp of creation.
    pub created: u64,
    /// Model used for the completion.
    pub model: String,
    /// List of completion choices.
    pub choices: Vec<ChatChoiceDelta>,
    /// Token usage (only in last chunk with stream_options).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
}

/// Available GLM models.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Model {
    /// GLM-4.7 - Latest model with 200K context.
    Glm47,
    /// GLM-4.7 Flash - Fast version.
    Glm47Flash,
    /// GLM-4.6.
    Glm46,
    /// GLM-4.5.
    Glm45,
    /// GLM-4.
    Glm4,
    /// GLM-4 Flash.
    Glm4Flash,
    /// GLM-4V - Vision model.
    Glm4V,
    /// GLM-4V Plus - Enhanced vision model.
    Glm4VPlus,
    /// CharGLM-3 - Character roleplay model.
    CharGlm3,
}

impl Model {
    /// Get the model ID string.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Glm47 => "glm-4.7",
            Self::Glm47Flash => "glm-4.7-flash",
            Self::Glm46 => "glm-4.6",
            Self::Glm45 => "glm-4.5",
            Self::Glm4 => "glm-4",
            Self::Glm4Flash => "glm-4-flash",
            Self::Glm4V => "glm-4v",
            Self::Glm4VPlus => "glm-4v-plus",
            Self::CharGlm3 => "charglm-3",
        }
    }
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<Model> for String {
    fn from(model: Model) -> Self {
        model.as_str().to_string()
    }
}
