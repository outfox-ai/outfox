//! Voice API implementation.

use crate::Client;
use crate::error::{ErrorResponse, Result, ZhipuError};
use crate::spec::voice::{
    VoiceCloneRequest, VoiceCloneResponse, VoiceDeleteRequest, VoiceDeleteResponse, VoiceListQuery,
    VoiceListResponse, VoiceType,
};

/// Simple URL encoding for query parameters.
fn url_encode(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '~' => {
                result.push(c);
            }
            _ => {
                for b in c.to_string().as_bytes() {
                    result.push_str(&format!("%{:02X}", b));
                }
            }
        }
    }
    result
}

/// Voice API.
pub struct Voice<'c> {
    client: &'c Client,
}

impl<'c> Voice<'c> {
    /// Create a new Voice API.
    pub(crate) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Clone a voice from an audio sample.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn clone(&self, request: VoiceCloneRequest) -> Result<VoiceCloneResponse> {
        let config = self.client.config();
        let url = config.url("/voice/clone");
        let headers = config.headers()?;

        let response = self
            .client
            .http_client()
            .post(&url)
            .headers(headers)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error: ErrorResponse = response.json().await?;
            return Err(ZhipuError::ApiError(error.error));
        }

        let body = response.json().await?;
        Ok(body)
    }

    /// List available voices.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn list(&self, query: Option<VoiceListQuery>) -> Result<VoiceListResponse> {
        let config = self.client.config();
        let mut url = config.url("/voice/list");
        let headers = config.headers()?;

        // Add query parameters
        if let Some(q) = query {
            let mut params = Vec::new();
            if let Some(name) = &q.voice_name {
                params.push(format!("voiceName={}", url_encode(name)));
            }
            if let Some(voice_type) = &q.voice_type {
                let type_str = match voice_type {
                    VoiceType::Official => "OFFICIAL",
                    VoiceType::Private => "PRIVATE",
                };
                params.push(format!("voiceType={}", type_str));
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

    /// List all available voices.
    pub async fn list_all(&self) -> Result<VoiceListResponse> {
        self.list(None).await
    }

    /// List official system voices.
    pub async fn list_official(&self) -> Result<VoiceListResponse> {
        self.list(Some(VoiceListQuery {
            voice_name: None,
            voice_type: Some(VoiceType::Official),
        }))
        .await
    }

    /// List private cloned voices.
    pub async fn list_private(&self) -> Result<VoiceListResponse> {
        self.list(Some(VoiceListQuery {
            voice_name: None,
            voice_type: Some(VoiceType::Private),
        }))
        .await
    }

    /// Delete a cloned voice.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    pub async fn delete(&self, voice: &str) -> Result<VoiceDeleteResponse> {
        let config = self.client.config();
        let url = config.url("/voice/delete");
        let headers = config.headers()?;

        let request = VoiceDeleteRequest {
            voice: voice.to_string(),
            request_id: None,
        };

        let response = self
            .client
            .http_client()
            .post(&url)
            .headers(headers)
            .json(&request)
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
