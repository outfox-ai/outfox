//! Files API implementation.

use bytes::Bytes;
use reqwest::multipart::{Form, Part};

use crate::Client;
use crate::error::{ErrorResponse, Result, ZhipuError};
use crate::spec::files::{
    CreateFileRequest, FileDeleted, FileObject, FilePurpose, ListFilesQuery, ListFilesResponse,
    UploadDetail,
};

/// Files API.
pub struct Files<'c> {
    client: &'c Client,
}

impl<'c> Files<'c> {
    /// Create a new Files API.
    pub(crate) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Upload a file.
    ///
    /// # Arguments
    ///
    /// * `file_data` - The file data as bytes.
    /// * `filename` - The filename.
    /// * `request` - The file upload request.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn create(
        &self,
        file_data: Bytes,
        filename: &str,
        request: CreateFileRequest,
    ) -> Result<FileObject> {
        let config = self.client.config();
        let url = config.url("/files");
        let headers = config.headers()?;

        let purpose_str = match request.purpose {
            FilePurpose::FineTune => "fine-tune",
            FilePurpose::Retrieval => "retrieval",
            FilePurpose::Batch => "batch",
            FilePurpose::VoiceCloneInput => "voice-clone-input",
        };

        let mut form = Form::new().text("purpose", purpose_str.to_string()).part(
            "file",
            Part::bytes(file_data.to_vec()).file_name(filename.to_string()),
        );

        if let Some(knowledge_id) = request.knowledge_id {
            form = form.text("knowledge_id", knowledge_id);
        }

        if let Some(sentence_size) = request.sentence_size {
            form = form.text("sentence_size", sentence_size.to_string());
        }

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

    /// Upload a file from URL(s).
    ///
    /// # Arguments
    ///
    /// * `upload_details` - List of upload details with URLs.
    /// * `purpose` - Purpose of the file.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn create_from_url(
        &self,
        upload_details: Vec<UploadDetail>,
        purpose: FilePurpose,
    ) -> Result<FileObject> {
        let config = self.client.config();
        let url = config.url("/files");
        let headers = config.headers()?;

        let body = serde_json::json!({
            "upload_detail": upload_details,
            "purpose": purpose,
        });

        let response = self
            .client
            .http_client()
            .post(&url)
            .headers(headers)
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error: ErrorResponse = response.json().await?;
            return Err(ZhipuError::ApiError(error.error));
        }

        let body = response.json().await?;
        Ok(body)
    }

    /// List files.
    ///
    /// # Arguments
    ///
    /// * `query` - Optional query parameters.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn list(&self, query: Option<ListFilesQuery>) -> Result<ListFilesResponse> {
        let config = self.client.config();
        let mut url = config.url("/files");
        let headers = config.headers()?;

        if let Some(q) = &query {
            let mut params = vec![];
            if let Some(purpose) = &q.purpose {
                params.push(format!("purpose={}", purpose));
            }
            if let Some(limit) = q.limit {
                params.push(format!("limit={}", limit));
            }
            if let Some(after) = &q.after {
                params.push(format!("after={}", after));
            }
            if let Some(order) = &q.order {
                params.push(format!("order={}", order));
            }
            if !params.is_empty() {
                url = format!("{}?{}", url, params.join("&"));
            }
        }

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

    /// Delete a file.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The ID of the file to delete.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn delete(&self, file_id: &str) -> Result<FileDeleted> {
        let config = self.client.config();
        let url = config.url(&format!("/files/{}", file_id));
        let headers = config.headers()?;

        let response = self
            .client
            .http_client()
            .delete(&url)
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

    /// Get file content.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The ID of the file.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn content(&self, file_id: &str) -> Result<Bytes> {
        let config = self.client.config();
        let url = config.url(&format!("/files/{}/content", file_id));
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

        let bytes = response.bytes().await?;
        Ok(bytes)
    }
}
