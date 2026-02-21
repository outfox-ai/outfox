//! Chat completion request and response types.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Role of a chat message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// System message providing context.
    System,
    /// User message.
    #[default]
    User,
    /// Assistant response.
    Assistant,
    /// Tool result message.
    Tool,
}

/// Detail level for image URL.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ImageUrlDetail {
    /// High detail processing.
    High,
    /// Low detail processing.
    Low,
    /// Automatic detail selection.
    #[default]
    Auto,
}

/// Reasoning effort level for thinking models.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEffort {
    /// Minimal reasoning.
    Minimal,
    /// Low reasoning.
    Low,
    /// Medium reasoning.
    Medium,
    /// High reasoning.
    High,
}

/// Image URL content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessageImageUrl {
    /// URL of the image.
    pub url: String,
    /// Detail level for processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<ImageUrlDetail>,
}

/// Video URL content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessageVideoUrl {
    /// URL of the video.
    pub url: String,
    /// Frames per second for video processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fps: Option<f64>,
}

/// Type of content part in a message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentPartType {
    /// Text content.
    Text,
    /// Image URL content.
    ImageUrl,
    /// Video URL content.
    VideoUrl,
}

/// A part of message content (for multimodal messages).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentPart {
    /// Type of content.
    #[serde(rename = "type")]
    pub content_type: ContentPartType,
    /// Text content (if type is text).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Image URL (if type is image_url).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<ChatMessageImageUrl>,
    /// Video URL (if type is video_url).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_url: Option<ChatMessageVideoUrl>,
}

impl ContentPart {
    /// Create a text content part.
    #[must_use]
    pub fn text<S: Into<String>>(text: S) -> Self {
        Self {
            content_type: ContentPartType::Text,
            text: Some(text.into()),
            image_url: None,
            video_url: None,
        }
    }

    /// Create an image URL content part.
    #[must_use]
    pub fn image_url<S: Into<String>>(url: S) -> Self {
        Self {
            content_type: ContentPartType::ImageUrl,
            text: None,
            image_url: Some(ChatMessageImageUrl {
                url: url.into(),
                detail: None,
            }),
            video_url: None,
        }
    }

    /// Create a video URL content part.
    #[must_use]
    pub fn video_url<S: Into<String>>(url: S) -> Self {
        Self {
            content_type: ContentPartType::VideoUrl,
            text: None,
            image_url: None,
            video_url: Some(ChatMessageVideoUrl {
                url: url.into(),
                fps: None,
            }),
        }
    }
}

/// Message content that can be either a string or multipart.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
    /// Simple text content.
    Text(String),
    /// Multipart content (for multimodal messages).
    Parts(Vec<ContentPart>),
}

impl From<&str> for MessageContent {
    fn from(s: &str) -> Self {
        MessageContent::Text(s.to_string())
    }
}

impl From<String> for MessageContent {
    fn from(s: String) -> Self {
        MessageContent::Text(s)
    }
}

impl From<Vec<ContentPart>> for MessageContent {
    fn from(parts: Vec<ContentPart>) -> Self {
        MessageContent::Parts(parts)
    }
}

/// Function call made by the assistant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    /// Name of the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Arguments as a JSON string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
}

/// Type of tool.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ToolType {
    /// Function tool.
    #[default]
    Function,
}

/// Tool call made by the assistant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// Unique ID of the tool call.
    pub id: String,
    /// Type of tool.
    #[serde(rename = "type")]
    pub tool_type: ToolType,
    /// Function call details.
    pub function: FunctionCall,
    /// Index of the tool call in streaming responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
}

/// A chat message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Role of the message sender.
    pub role: Role,
    /// Content of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<MessageContent>,
    /// Reasoning content for thinking models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_content: Option<String>,
    /// Name of the participant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Function call (deprecated, use tool_calls).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCall>,
    /// Tool calls made by the assistant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    /// Tool call ID (for tool messages).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

impl ChatMessage {
    /// Create a system message.
    #[must_use]
    pub fn system<S: Into<String>>(content: S) -> Self {
        Self {
            role: Role::System,
            content: Some(MessageContent::Text(content.into())),
            reasoning_content: None,
            name: None,
            function_call: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }

    /// Create a user message.
    #[must_use]
    pub fn user<C: Into<MessageContent>>(content: C) -> Self {
        Self {
            role: Role::User,
            content: Some(content.into()),
            reasoning_content: None,
            name: None,
            function_call: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }

    /// Create an assistant message.
    #[must_use]
    pub fn assistant<S: Into<String>>(content: S) -> Self {
        Self {
            role: Role::Assistant,
            content: Some(MessageContent::Text(content.into())),
            reasoning_content: None,
            name: None,
            function_call: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }

    /// Create a tool result message.
    #[must_use]
    pub fn tool<S: Into<String>>(tool_call_id: S, content: S) -> Self {
        Self {
            role: Role::Tool,
            content: Some(MessageContent::Text(content.into())),
            reasoning_content: None,
            name: None,
            function_call: None,
            tool_calls: None,
            tool_call_id: Some(tool_call_id.into()),
        }
    }
}

/// Type of thinking mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ThinkingType {
    /// Enable thinking.
    Enabled,
    /// Disable thinking.
    #[default]
    Disabled,
    /// Auto thinking.
    Auto,
}

/// Thinking configuration for reasoning models.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thinking {
    /// Thinking type.
    #[serde(rename = "type")]
    pub thinking_type: ThinkingType,
}

/// Function definition for tools.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDefinition {
    /// Name of the function.
    pub name: String,
    /// Description of the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// JSON Schema for function parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}

/// Tool definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// Type of tool.
    #[serde(rename = "type")]
    pub tool_type: ToolType,
    /// Function definition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<FunctionDefinition>,
}

impl Tool {
    /// Create a function tool.
    #[must_use]
    pub fn function<S: Into<String>>(name: S) -> Self {
        Self {
            tool_type: ToolType::Function,
            function: Some(FunctionDefinition {
                name: name.into(),
                description: None,
                parameters: None,
            }),
        }
    }

    /// Set the function description.
    #[must_use]
    pub fn with_description<S: Into<String>>(mut self, description: S) -> Self {
        if let Some(ref mut f) = self.function {
            f.description = Some(description.into());
        }
        self
    }

    /// Set the function parameters schema.
    #[must_use]
    pub fn with_parameters(mut self, parameters: serde_json::Value) -> Self {
        if let Some(ref mut f) = self.function {
            f.parameters = Some(parameters);
        }
        self
    }
}

/// Tool choice specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolChoice {
    /// String value ("auto", "none", "required").
    String(String),
    /// Specific tool choice.
    Tool {
        /// Type of tool.
        #[serde(rename = "type")]
        tool_type: ToolType,
        /// Function to call.
        function: ToolChoiceFunction,
    },
}

/// Function choice for tool choice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolChoiceFunction {
    /// Name of the function.
    pub name: String,
}

impl ToolChoice {
    /// Auto tool choice.
    pub const AUTO: Self = ToolChoice::String(String::new());
    /// No tool choice.
    pub const NONE: Self = ToolChoice::String(String::new());
    /// Required tool choice.
    pub const REQUIRED: Self = ToolChoice::String(String::new());

    /// Create auto tool choice.
    #[must_use]
    pub fn auto() -> Self {
        ToolChoice::String("auto".to_string())
    }

    /// Create none tool choice.
    #[must_use]
    pub fn none() -> Self {
        ToolChoice::String("none".to_string())
    }

    /// Create required tool choice.
    #[must_use]
    pub fn required() -> Self {
        ToolChoice::String("required".to_string())
    }

    /// Create a specific function choice.
    #[must_use]
    pub fn function<S: Into<String>>(name: S) -> Self {
        ToolChoice::Tool {
            tool_type: ToolType::Function,
            function: ToolChoiceFunction { name: name.into() },
        }
    }
}

/// Stream options.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StreamOptions {
    /// Include usage in the final chunk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_usage: Option<bool>,
    /// Include usage in each chunk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_include_usage: Option<bool>,
}

/// Response format type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormatType {
    /// Plain text response.
    Text,
    /// JSON object response.
    JsonObject,
    /// JSON schema response.
    JsonSchema,
}

/// JSON schema for response format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseFormatJsonSchema {
    /// Name of the schema.
    pub name: String,
    /// Description of the schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// JSON Schema object.
    pub schema: serde_json::Value,
    /// Strict schema adherence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

/// Response format specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseFormat {
    /// Type of response format.
    #[serde(rename = "type")]
    pub format_type: ResponseFormatType,
    /// JSON schema (for json_schema type).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_schema: Option<ResponseFormatJsonSchema>,
}

impl ResponseFormat {
    /// Create a text response format.
    #[must_use]
    pub fn text() -> Self {
        Self {
            format_type: ResponseFormatType::Text,
            json_schema: None,
        }
    }

    /// Create a JSON object response format.
    #[must_use]
    pub fn json_object() -> Self {
        Self {
            format_type: ResponseFormatType::JsonObject,
            json_schema: None,
        }
    }

    /// Create a JSON schema response format.
    #[must_use]
    pub fn json_schema<S: Into<String>>(name: S, schema: serde_json::Value) -> Self {
        Self {
            format_type: ResponseFormatType::JsonSchema,
            json_schema: Some(ResponseFormatJsonSchema {
                name: name.into(),
                description: None,
                schema,
                strict: None,
            }),
        }
    }
}

/// Chat completion request.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(name = "CreateChatCompletionRequestArgs", setter(into, strip_option))]
pub struct CreateChatCompletionRequest {
    /// Model ID to use.
    pub model: String,
    /// Messages in the conversation.
    pub messages: Vec<ChatMessage>,
    /// Maximum tokens to generate.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    /// Maximum completion tokens.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<i32>,
    /// Sampling temperature (0-2).
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Nucleus sampling probability.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// Whether to stream the response.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Stop sequences.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    /// Frequency penalty (-2 to 2).
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    /// Presence penalty (-2 to 2).
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    /// Repetition penalty.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repetition_penalty: Option<f32>,
    /// Logit bias map.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<std::collections::HashMap<String, i32>>,
    /// Whether to return log probabilities.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,
    /// Number of top log probabilities to return.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<i32>,
    /// User identifier.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Tools available to the model.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    /// Tool choice specification.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    /// Whether to run tool calls in parallel.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    /// Stream options.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,
    /// Number of completions to generate.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    /// Response format specification.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    /// Service tier.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>,
    /// Thinking configuration.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<Thinking>,
    /// Reasoning effort level.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ReasoningEffort>,
}

/// Reason for completion finish.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    /// Completed normally.
    Stop,
    /// Hit token limit.
    Length,
    /// Made a function call.
    FunctionCall,
    /// Made tool calls.
    ToolCalls,
    /// Content filtered.
    ContentFilter,
}

/// Top log probability entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopLogProb {
    /// Token text.
    pub token: String,
    /// Log probability.
    pub logprob: f64,
    /// Bytes of the token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
}

/// Log probability entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogProb {
    /// Token text.
    pub token: String,
    /// Log probability.
    pub logprob: f64,
    /// Bytes of the token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
    /// Top log probabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<Vec<TopLogProb>>,
}

/// Log probabilities container.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogProbs {
    /// Content log probabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<LogProb>>,
}

/// Token usage information.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Usage {
    /// Number of tokens in the prompt.
    pub prompt_tokens: i64,
    /// Number of tokens in the completion.
    pub completion_tokens: i64,
    /// Total tokens used.
    pub total_tokens: i64,
}

/// Moderation hit type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModerationHitType {
    /// Violence detected.
    Violence,
    /// Severe violation detected.
    SevereViolation,
}

/// Chat completion choice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionChoice {
    /// Index of this choice.
    pub index: i32,
    /// Message content.
    pub message: ChatMessage,
    /// Reason the completion finished.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<FinishReason>,
    /// Moderation hit type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_hit_type: Option<ModerationHitType>,
    /// Log probabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<LogProbs>,
}

/// Chat completion response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChatCompletionResponse {
    /// Unique ID of the completion.
    pub id: String,
    /// Object type.
    pub object: String,
    /// Creation timestamp.
    pub created: i64,
    /// Model used.
    pub model: String,
    /// Service tier (if applicable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>,
    /// Completion choices.
    pub choices: Vec<ChatCompletionChoice>,
    /// Token usage.
    pub usage: Usage,
}

/// Delta content in streaming response.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChatCompletionStreamDelta {
    /// Content text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Role of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    /// Reasoning content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_content: Option<String>,
    /// Function call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCall>,
    /// Tool calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

/// Streaming choice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionStreamChoice {
    /// Index of this choice.
    pub index: i32,
    /// Delta content.
    pub delta: ChatCompletionStreamDelta,
    /// Log probabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<LogProbs>,
    /// Finish reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<FinishReason>,
    /// Moderation hit type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_hit_type: Option<ModerationHitType>,
}

/// Streaming chat completion response chunk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionChunk {
    /// Unique ID of the completion.
    pub id: String,
    /// Object type.
    pub object: String,
    /// Creation timestamp.
    pub created: i64,
    /// Model used.
    pub model: String,
    /// Service tier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>,
    /// Streaming choices.
    pub choices: Vec<ChatCompletionStreamChoice>,
    /// Token usage (only in final chunk if requested).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
}
