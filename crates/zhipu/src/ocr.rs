//! OCR API implementation.

use bytes::Bytes;
use reqwest::multipart::{Form, Part};

use crate::Client;
use crate::error::{ErrorResponse, Result, ZhipuError};
use crate::spec::ocr::{OcrResponse, OcrToolType};

/// OCR API.
pub struct Ocr<'c> {
    client: &'c Client,
}

impl<'c> Ocr<'c> {
    /// Create a new OCR API.
    pub(crate) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Perform handwriting OCR on an image.
    ///
    /// # Arguments
    ///
    /// * `file_data` - The image file data.
    /// * `filename` - The filename.
    /// * `language_type` - Optional language type.
    /// * `probability` - Whether to include probability scores.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn handwriting(
        &self,
        file_data: Bytes,
        filename: &str,
        language_type: Option<&str>,
        probability: Option<bool>,
    ) -> Result<OcrResponse> {
        let config = self.client.config();
        let url = config.url("/files/ocr");
        let headers = config.headers()?;

        let tool_type = match OcrToolType::HandWrite {
            OcrToolType::HandWrite => "hand_write",
        };

        let mut form = Form::new().text("tool_type", tool_type.to_string()).part(
            "file",
            Part::bytes(file_data.to_vec()).file_name(filename.to_string()),
        );

        if let Some(lang) = language_type {
            form = form.text("language_type", lang.to_string());
        }

        if let Some(prob) = probability {
            form = form.text("probability", prob.to_string());
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
}
