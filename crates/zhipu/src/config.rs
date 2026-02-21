//! Configuration for Zhipu AI API client.

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

use crate::error::ZhipuError;

/// Default API base URL.
pub const ZHIPU_API_BASE: &str = "https://open.bigmodel.cn/api/paas/v4";

/// Configuration for Zhipu AI API.
#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct ZhipuConfig {
    /// API key for authentication.
    api_key: SecretString,
    /// API base URL.
    api_base: String,
}

impl Default for ZhipuConfig {
    fn default() -> Self {
        Self {
            api_key: default_api_key().into(),
            api_base: default_api_base(),
        }
    }
}

fn default_api_key() -> String {
    std::env::var("ZHIPUAI_API_KEY")
        .or_else(|_| std::env::var("ZHIPU_API_KEY"))
        .unwrap_or_default()
}

fn default_api_base() -> String {
    std::env::var("ZHIPUAI_BASE_URL")
        .or_else(|_| std::env::var("ZHIPU_API_BASE"))
        .unwrap_or_else(|_| ZHIPU_API_BASE.to_string())
}

impl ZhipuConfig {
    /// Create a new configuration with default values from environment variables.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the API key.
    #[must_use]
    pub fn with_api_key<S: Into<String>>(mut self, api_key: S) -> Self {
        self.api_key = SecretString::from(api_key.into());
        self
    }

    /// Set the API base URL.
    #[must_use]
    pub fn with_api_base<S: Into<String>>(mut self, api_base: S) -> Self {
        self.api_base = api_base.into();
        self
    }

    /// Get the API key (exposed secret).
    #[must_use]
    pub fn api_key(&self) -> &str {
        self.api_key.expose_secret()
    }

    /// Get the API base URL.
    #[must_use]
    pub fn api_base(&self) -> &str {
        &self.api_base
    }

    /// Build the full URL for an endpoint.
    #[must_use]
    pub fn url(&self, path: &str) -> String {
        format!("{}{}", self.api_base, path)
    }

    /// Build request headers.
    pub fn headers(&self) -> Result<HeaderMap, ZhipuError> {
        let mut headers = HeaderMap::new();

        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.api_key.expose_secret()))
                .map_err(|e| ZhipuError::InvalidArgument(format!("invalid api_key: {e}")))?,
        );

        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        Ok(headers)
    }
}
