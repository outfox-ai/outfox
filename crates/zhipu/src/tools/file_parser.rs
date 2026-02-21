//! File parsing API implementation.

use bytes::Bytes;
use reqwest::multipart::{Form, Part};

use crate::Client;
use crate::error::{ErrorResponse, Result, ZhipuError};
use crate::spec::tools::{
    FileParseResponse, FileParseResultResponse, ParseResultFormat, ParserToolType,
};

/// File parsing API.
pub struct FileParser<'c> {
    client: &'c Client,
}

impl<'c> FileParser<'c> {
    /// Create a new FileParser API.
    pub(crate) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Create a file parsing task.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn create(
        &self,
        file_data: Bytes,
        filename: &str,
        tool_type: ParserToolType,
    ) -> Result<FileParseResponse> {
        let config = self.client.config();
        let url = config.url("/files/parser/create");
        let headers = config.headers()?;

        let tool_type_str = match tool_type {
            ParserToolType::Lite => "lite",
            ParserToolType::Expert => "expert",
            ParserToolType::Prime => "prime",
        };

        let part = Part::bytes(file_data.to_vec())
            .file_name(filename.to_string())
            .mime_str("application/octet-stream")
            .map_err(|e| ZhipuError::InvalidArgument(e.to_string()))?;

        let form = Form::new()
            .part("file", part)
            .text("tool_type", tool_type_str.to_string());

        let response = self
            .client
            .http_client()
            .post(&url)
            .headers(headers)
            .multipart(form)
            .send()
            .await?;

        if !response.status().is_success() {
            let error: ErrorResponse = response.json().await?;
            return Err(ZhipuError::ApiError(error.error));
        }

        let body = response.json().await?;
        Ok(body)
    }

    /// Create a file parsing task from a file path.
    pub async fn create_from_file<P: AsRef<std::path::Path>>(
        &self,
        path: P,
        tool_type: ParserToolType,
    ) -> Result<FileParseResponse> {
        let path = path.as_ref();
        let data = tokio::fs::read(path)
            .await
            .map_err(|e| ZhipuError::FileError(e.to_string()))?;
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file")
            .to_string();

        self.create(Bytes::from(data), &filename, tool_type).await
    }

    /// Get the result of a file parsing task as text.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn get_result_text(&self, task_id: &str) -> Result<FileParseResultResponse> {
        self.get_result(task_id, ParseResultFormat::Text).await
    }

    /// Get the result of a file parsing task as download link.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn get_result_download(&self, task_id: &str) -> Result<FileParseResultResponse> {
        self.get_result(task_id, ParseResultFormat::DownloadLink)
            .await
    }

    /// Get the result of a file parsing task.
    async fn get_result(
        &self,
        task_id: &str,
        format_type: ParseResultFormat,
    ) -> Result<FileParseResultResponse> {
        let config = self.client.config();
        let format_str = match format_type {
            ParseResultFormat::Text => "text",
            ParseResultFormat::DownloadLink => "download_link",
        };
        let url = config.url(&format!("/files/parser/result/{}/{}", task_id, format_str));
        let headers = config.headers()?;

        let response = self
            .client
            .http_client()
            .get(&url)
            .headers(headers)
            .send()
            .await?;

        if !response.status().is_success() {
            let error: ErrorResponse = response.json().await?;
            return Err(ZhipuError::ApiError(error.error));
        }

        let body = response.json().await?;
        Ok(body)
    }

    /// Parse a file and wait for results (convenience method).
    ///
    /// This creates a parsing task and polls until completion.
    pub async fn parse_file<P: AsRef<std::path::Path>>(
        &self,
        path: P,
        tool_type: ParserToolType,
    ) -> Result<String> {
        let response = self.create_from_file(path, tool_type).await?;
        let task_id = response.task_id;

        // Poll for results
        loop {
            let result = self.get_result_text(&task_id).await?;
            match result.status {
                crate::spec::tools::ParseStatus::Succeeded => {
                    return Ok(result.content.unwrap_or_default());
                }
                crate::spec::tools::ParseStatus::Failed => {
                    return Err(ZhipuError::InvalidArgument(format!(
                        "File parsing failed: {}",
                        result.message
                    )));
                }
                crate::spec::tools::ParseStatus::Processing => {
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
        }
    }
}
