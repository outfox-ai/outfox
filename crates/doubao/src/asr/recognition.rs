//! HTTP-based speech recognition implementation.

use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue};

use crate::Client;
use crate::error::{DoubaoError, Result};
use crate::spec::asr::{
    ASR_API_BASE, ASR_FLASH_PATH, ASR_QUERY_PATH, ASR_SUBMIT_PATH, AsrRequestConfig, AsrResponse,
    AsrUserInfo, FlashRecognizeRequest, HEADER_ACCESS_KEY, HEADER_APP_KEY, HEADER_LOG_ID,
    HEADER_MESSAGE, HEADER_REQUEST_ID, HEADER_RESOURCE_ID, HEADER_SEQUENCE, HEADER_STATUS_CODE,
    QueryResponse, RESOURCE_ID_BIGASR, RESOURCE_ID_BIGASR_TURBO, SubmitTaskRequest, TaskStatus,
};

/// File-based speech recognition API.
///
/// Provides two modes:
/// - Standard: Submit task + poll for results
/// - Flash/Turbo: Single request with immediate result
pub struct Recognition<'c> {
    client: &'c Client,
    http_client: reqwest::Client,
}

impl<'c> Recognition<'c> {
    /// Create a new Recognition API.
    pub(crate) fn new(client: &'c Client) -> Self {
        Self {
            client,
            http_client: reqwest::Client::new(),
        }
    }

    /// Build common headers for ASR requests.
    fn build_headers(&self, resource_id: &str, request_id: &str) -> Result<HeaderMap> {
        let config = self.client.config();
        let mut headers = HeaderMap::new();

        headers.insert(
            HEADER_APP_KEY,
            HeaderValue::from_str(config.app_id())
                .map_err(|e| DoubaoError::InvalidArgument(format!("invalid app_id: {e}")))?,
        );
        headers.insert(
            HEADER_ACCESS_KEY,
            HeaderValue::from_str(config.access_token())
                .map_err(|e| DoubaoError::InvalidArgument(format!("invalid access_token: {e}")))?,
        );
        headers.insert(
            HEADER_RESOURCE_ID,
            HeaderValue::from_str(resource_id)
                .map_err(|e| DoubaoError::InvalidArgument(format!("invalid resource_id: {e}")))?,
        );
        headers.insert(
            HEADER_REQUEST_ID,
            HeaderValue::from_str(request_id)
                .map_err(|e| DoubaoError::InvalidArgument(format!("invalid request_id: {e}")))?,
        );
        headers.insert(HEADER_SEQUENCE, HeaderValue::from_static("-1"));

        Ok(headers)
    }

    /// Submit a recognition task (standard version).
    ///
    /// Returns the task ID for querying results.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn submit(&self, request: SubmitTaskRequest) -> Result<String> {
        let task_id = uuid::Uuid::new_v4().to_string();
        let headers = self.build_headers(RESOURCE_ID_BIGASR, &task_id)?;

        let url = format!("{}{}", ASR_API_BASE, ASR_SUBMIT_PATH);

        let response = self
            .http_client
            .post(&url)
            .headers(headers)
            .json(&request)
            .send()
            .await
            .map_err(|e| DoubaoError::Protocol(format!("request failed: {e}")))?;

        let status_code = response
            .headers()
            .get(HEADER_STATUS_CODE)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0);

        let message = response
            .headers()
            .get(HEADER_MESSAGE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        if status_code != 20000000 && status_code != 20000001 && status_code != 20000002 {
            return Err(DoubaoError::Protocol(format!(
                "submit failed: {} (code: {})",
                message, status_code
            )));
        }

        tracing::debug!("Task submitted: {}", task_id);
        Ok(task_id)
    }

    /// Query the result of a submitted task.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn query(&self, task_id: &str) -> Result<QueryResponse> {
        let headers = self.build_headers(RESOURCE_ID_BIGASR, task_id)?;

        let url = format!("{}{}", ASR_API_BASE, ASR_QUERY_PATH);

        let response = self
            .http_client
            .post(&url)
            .headers(headers)
            .json(&serde_json::json!({}))
            .send()
            .await
            .map_err(|e| DoubaoError::Protocol(format!("request failed: {e}")))?;

        let status_code = response
            .headers()
            .get(HEADER_STATUS_CODE)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0);

        let message = response
            .headers()
            .get(HEADER_MESSAGE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        let log_id = response
            .headers()
            .get(HEADER_LOG_ID)
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        let status = TaskStatus::from_code(status_code);

        let result = if status.is_completed() {
            let body = response
                .json::<AsrResponse>()
                .await
                .map_err(|e| DoubaoError::Protocol(format!("failed to parse response: {e}")))?;
            Some(body)
        } else {
            None
        };

        Ok(QueryResponse {
            status,
            message,
            log_id,
            result,
        })
    }

    /// Submit a task and wait for completion.
    ///
    /// Polls the API until the task is complete or an error occurs.
    ///
    /// # Arguments
    ///
    /// * `request` - The recognition request
    /// * `poll_interval` - Time to wait between polling attempts
    /// * `max_attempts` - Maximum number of polling attempts
    ///
    /// # Errors
    ///
    /// Returns an error if the task fails or times out.
    pub async fn submit_and_wait(
        &self,
        request: SubmitTaskRequest,
        poll_interval: Duration,
        max_attempts: u32,
    ) -> Result<AsrResponse> {
        let task_id = self.submit(request).await?;

        for attempt in 0..max_attempts {
            tokio::time::sleep(poll_interval).await;

            let response = self.query(&task_id).await?;

            match response.status {
                TaskStatus::Success | TaskStatus::Silent => {
                    return response.result.ok_or_else(|| {
                        DoubaoError::Protocol("no result in completed response".to_string())
                    });
                }
                TaskStatus::Processing | TaskStatus::InQueue => {
                    tracing::debug!(
                        "Task {} still pending (attempt {}/{})",
                        task_id,
                        attempt + 1,
                        max_attempts
                    );
                    continue;
                }
                TaskStatus::Error(code) => {
                    return Err(DoubaoError::Protocol(format!(
                        "task failed: {} (code: {})",
                        response.message, code
                    )));
                }
            }
        }

        Err(DoubaoError::Timeout)
    }

    /// Recognize audio using the flash/turbo API (single request).
    ///
    /// This is faster than the standard submit + query flow but has size limits.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn flash(&self, request: FlashRecognizeRequest) -> Result<AsrResponse> {
        let task_id = uuid::Uuid::new_v4().to_string();
        let headers = self.build_headers(RESOURCE_ID_BIGASR_TURBO, &task_id)?;

        let url = format!("{}{}", ASR_API_BASE, ASR_FLASH_PATH);

        let response = self
            .http_client
            .post(&url)
            .headers(headers)
            .json(&request)
            .send()
            .await
            .map_err(|e| DoubaoError::Protocol(format!("request failed: {e}")))?;

        let status_code = response
            .headers()
            .get(HEADER_STATUS_CODE)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0);

        let message = response
            .headers()
            .get(HEADER_MESSAGE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        let status = TaskStatus::from_code(status_code);

        if status.is_error() {
            return Err(DoubaoError::Protocol(format!(
                "flash recognition failed: {} (code: {})",
                message, status_code
            )));
        }

        let response = response.text().await.unwrap_or_default();
        println!("Flash response text: {}", response);
        let asr_response: AsrResponse = serde_json::from_str(&response).map_err(|e| {
            DoubaoError::Protocol(format!(
                "failed to parse response: {e} - response: {}",
                response
            ))
        })?;

        Ok(asr_response)
    }

    /// Recognize audio from a URL using the standard API.
    ///
    /// Convenience method that creates a request from a URL.
    pub async fn recognize_url(&self, url: &str, user_id: &str) -> Result<AsrResponse> {
        let request = SubmitTaskRequest {
            user: AsrUserInfo {
                uid: user_id.to_string(),
            },
            audio: crate::spec::asr::AsrAudioConfig {
                url: Some(url.to_string()),
                ..Default::default()
            },
            request: AsrRequestConfig {
                model_name: Some("bigmodel".to_string()),
                enable_itn: Some(true),
                ..Default::default()
            },
            ..Default::default()
        };

        self.submit_and_wait(request, Duration::from_secs(2), 60)
            .await
    }

    /// Recognize audio from a URL using the flash API.
    ///
    /// Convenience method that creates a request from a URL.
    pub async fn flash_url(&self, url: &str, user_id: &str) -> Result<AsrResponse> {
        let request = FlashRecognizeRequest {
            user: AsrUserInfo {
                uid: user_id.to_string(),
            },
            audio: crate::spec::asr::AsrAudioConfig {
                url: Some(url.to_string()),
                ..Default::default()
            },
            request: AsrRequestConfig {
                model_name: Some("bigmodel".to_string()),
                ..Default::default()
            },
        };

        self.flash(request).await
    }

    /// Recognize audio from bytes using the flash API.
    ///
    /// Convenience method that encodes bytes to base64.
    pub async fn flash_bytes(&self, data: &[u8], user_id: &str) -> Result<AsrResponse> {
        use base64::Engine;
        let encoded = base64::engine::general_purpose::STANDARD.encode(data);

        let request = FlashRecognizeRequest {
            user: AsrUserInfo {
                uid: user_id.to_string(),
            },
            audio: crate::spec::asr::AsrAudioConfig {
                data: Some(encoded),
                ..Default::default()
            },
            request: AsrRequestConfig {
                model_name: Some("bigmodel".to_string()),
                ..Default::default()
            },
        };

        self.flash(request).await
    }
}
