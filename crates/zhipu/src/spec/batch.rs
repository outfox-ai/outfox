//! Batch API request and response types.

use std::collections::HashMap;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Batch endpoint type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BatchEndpoint {
    /// Chat completions endpoint.
    #[serde(rename = "/v1/chat/completions")]
    ChatCompletions,
    /// Embeddings endpoint.
    #[serde(rename = "/v1/embeddings")]
    Embeddings,
}

/// Batch status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BatchStatus {
    /// Validating input.
    Validating,
    /// Batch failed.
    Failed,
    /// Batch in progress.
    InProgress,
    /// Finalizing batch.
    Finalizing,
    /// Batch completed.
    Completed,
    /// Batch expired.
    Expired,
    /// Batch is being cancelled.
    Cancelling,
    /// Batch was cancelled.
    Cancelled,
}

/// Batch create request.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(name = "CreateBatchRequestArgs", setter(into, strip_option))]
pub struct CreateBatchRequest {
    /// Endpoint for the batch.
    pub endpoint: BatchEndpoint,
    /// Input file ID.
    pub input_file_id: String,
    /// Completion window (optional).
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_window: Option<String>,
    /// Metadata for the batch.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// Whether to auto-delete input file.
    #[builder(default = "Some(true)")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_delete_input_file: Option<bool>,
}

/// Batch error.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchError {
    /// Error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Error message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Parameter that caused the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
    /// Line number in input file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i32>,
}

/// Batch errors container.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchErrors {
    /// Object type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    /// List of errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<BatchError>>,
}

/// Batch request counts.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchRequestCounts {
    /// Total requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// Completed requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<i32>,
    /// Failed requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<i32>,
}

/// Batch object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Batch {
    /// Batch identifier.
    pub id: String,
    /// Object type.
    pub object: String,
    /// Endpoint for the batch.
    pub endpoint: String,
    /// Input file ID.
    pub input_file_id: String,
    /// Completion window.
    pub completion_window: String,
    /// Status of the batch.
    pub status: BatchStatus,
    /// Creation timestamp.
    pub created_at: i64,
    /// Output file ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_file_id: Option<String>,
    /// Error file ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_file_id: Option<String>,
    /// In-progress timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_progress_at: Option<i64>,
    /// Expiration timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    /// Finalizing timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finalizing_at: Option<i64>,
    /// Completed timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<i64>,
    /// Failed timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<i64>,
    /// Expired timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<i64>,
    /// Cancelling timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelling_at: Option<i64>,
    /// Cancelled timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<i64>,
    /// Request counts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_counts: Option<BatchRequestCounts>,
    /// Metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// Errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<BatchErrors>,
}

/// List batches query parameters.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(name = "ListBatchesQueryArgs", setter(into, strip_option), default)]
pub struct ListBatchesQuery {
    /// Limit number of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

/// List batches response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBatchesResponse {
    /// Object type.
    pub object: String,
    /// List of batches.
    pub data: Vec<Batch>,
    /// First ID in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,
    /// Last ID in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,
    /// Whether there are more batches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}
