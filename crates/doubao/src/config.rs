//! Configuration for Doubao API client.

use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

/// Default TTS WebSocket API base URL (v3 bidirectional).
pub const DOUBAO_TTS_WS_BASE: &str = "wss://openspeech.bytedance.com/api/v3/tts/bidirection";

/// Default TTS HTTP API base URL (v3 unidirectional streaming).
pub const DOUBAO_TTS_HTTP_V3_BASE: &str = "https://openspeech.bytedance.com/api/v3/tts/unidirectional";

/// Default TTS WebSocket API base URL (v3 unidirectional streaming).
pub const DOUBAO_TTS_WS_V3_UNI_BASE: &str = "wss://openspeech.bytedance.com/api/v3/tts/unidirectional";

/// Default HTTP API base URL for arkruntime APIs (Chat, Embeddings, Images, etc.).
pub const DOUBAO_HTTP_BASE: &str = "https://ark.cn-beijing.volces.com/api/v3";

/// Configuration for Doubao API.
#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct DoubaoConfig {
    /// Application ID.
    app_id: String,
    /// API key for authentication.
    api_key: SecretString,
    /// Access token for authentication.
    access_token: SecretString,
    /// Resource ID (e.g., "seed-tts-2.0").
    resource_id: String,
    /// WebSocket base URL for TTS v3 bidirectional.
    tts_ws_base: String,
    /// WebSocket base URL for TTS v3 unidirectional streaming.
    tts_ws_v3_uni_base: String,
    /// HTTP base URL for TTS v3 unidirectional streaming.
    tts_http_v3_base: String,
    /// HTTP base URL for arkruntime APIs.
    http_base: String,
    /// Voice type/speaker ID.
    voice_type: String,
    /// Cluster (volcano_tts or volcano_mega for cloned voices).
    cluster: String,
}

impl Default for DoubaoConfig {
    fn default() -> Self {
        Self {
            app_id: default_app_id(),
            api_key: default_api_key().into(),
            access_token: default_access_token().into(),
            resource_id: default_resource_id(),
            tts_ws_base: DOUBAO_TTS_WS_BASE.to_string(),
            tts_ws_v3_uni_base: DOUBAO_TTS_WS_V3_UNI_BASE.to_string(),
            tts_http_v3_base: DOUBAO_TTS_HTTP_V3_BASE.to_string(),
            http_base: default_http_base(),
            voice_type: default_voice_type(),
            cluster: default_cluster(),
        }
    }
}

fn default_app_id() -> String {
    std::env::var("DOUBAO_APP_ID").unwrap_or_default()
}

fn default_api_key() -> String {
    std::env::var("DOUBAO_API_KEY").unwrap_or_default()
}

fn default_access_token() -> String {
    std::env::var("DOUBAO_ACCESS_TOKEN").unwrap_or_default()
}

fn default_resource_id() -> String {
    std::env::var("DOUBAO_RESOURCE_ID").unwrap_or_else(|_| "seed-tts-2.0".to_string())
}

fn default_http_base() -> String {
    std::env::var("DOUBAO_HTTP_BASE").unwrap_or_else(|_| DOUBAO_HTTP_BASE.to_string())
}

fn default_voice_type() -> String {
    std::env::var("DOUBAO_VOICE_TYPE").unwrap_or_else(|_| "zh_female_vv_uranus_bigtts".to_string())
}

fn default_cluster() -> String {
    std::env::var("DOUBAO_CLUSTER").unwrap_or_else(|_| "volcano_tts".to_string())
}

impl DoubaoConfig {
    /// Create a new configuration with default values from environment variables.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the application ID.
    #[must_use]
    pub fn with_app_id<S: Into<String>>(mut self, app_id: S) -> Self {
        self.app_id = app_id.into();
        self
    }

    /// Set the API key.
    #[must_use]
    pub fn with_api_key<S: Into<String>>(mut self, api_key: S) -> Self {
        self.api_key = SecretString::from(api_key.into());
        self
    }

    /// Set the access token.
    #[must_use]
    pub fn with_access_token<S: Into<String>>(mut self, access_token: S) -> Self {
        self.access_token = SecretString::from(access_token.into());
        self
    }

    /// Set the resource ID.
    #[must_use]
    pub fn with_resource_id<S: Into<String>>(mut self, resource_id: S) -> Self {
        self.resource_id = resource_id.into();
        self
    }

    /// Set the WebSocket base URL (v3 bidirectional).
    #[must_use]
    pub fn with_tts_ws_base<S: Into<String>>(mut self, tts_ws_base: S) -> Self {
        self.tts_ws_base = tts_ws_base.into();
        self
    }

    /// Set the WebSocket base URL (v3 unidirectional streaming).
    #[must_use]
    pub fn with_tts_ws_v3_uni_base<S: Into<String>>(mut self, tts_ws_v3_uni_base: S) -> Self {
        self.tts_ws_v3_uni_base = tts_ws_v3_uni_base.into();
        self
    }

    /// Set the HTTP base URL (v3 unidirectional streaming TTS).
    #[must_use]
    pub fn with_tts_http_v3_base<S: Into<String>>(mut self, tts_http_v3_base: S) -> Self {
        self.tts_http_v3_base = tts_http_v3_base.into();
        self
    }

    /// Set the voice type.
    #[must_use]
    pub fn with_voice_type<S: Into<String>>(mut self, voice_type: S) -> Self {
        self.voice_type = voice_type.into();
        self
    }

    /// Set the cluster (volcano_tts or volcano_mega for cloned voices).
    #[must_use]
    pub fn with_cluster<S: Into<String>>(mut self, cluster: S) -> Self {
        self.cluster = cluster.into();
        self
    }

    /// Get the application ID.
    #[must_use]
    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    /// Get the API key (exposed secret).
    #[must_use]
    pub fn api_key(&self) -> &str {
        self.api_key.expose_secret()
    }

    /// Get the access token (exposed secret).
    #[must_use]
    pub fn access_token(&self) -> &str {
        self.access_token.expose_secret()
    }

    /// Get the resource ID.
    #[must_use]
    pub fn resource_id(&self) -> &str {
        &self.resource_id
    }

    /// Get the WebSocket base URL (v3 bidirectional).
    #[must_use]
    pub fn tts_ws_base(&self) -> &str {
        &self.tts_ws_base
    }

    /// Get the WebSocket base URL (v3 unidirectional streaming).
    #[must_use]
    pub fn tts_ws_v3_uni_base(&self) -> &str {
        &self.tts_ws_v3_uni_base
    }

    /// Get the HTTP base URL (v3 unidirectional streaming TTS).
    #[must_use]
    pub fn tts_http_v3_base(&self) -> &str {
        &self.tts_http_v3_base
    }

    /// Get the voice type.
    #[must_use]
    pub fn voice_type(&self) -> &str {
        &self.voice_type
    }

    /// Get the cluster.
    #[must_use]
    pub fn cluster(&self) -> &str {
        &self.cluster
    }

    /// Build the Authorization header value for WebSocket TTS/ASR.
    #[must_use]
    pub fn authorization(&self) -> String {
        format!("Bearer;{}", self.api_key.expose_secret())
    }

    /// Set the HTTP base URL.
    #[must_use]
    pub fn with_http_base<S: Into<String>>(mut self, http_base: S) -> Self {
        let base = http_base.into();
        self.http_base = base.trim_end_matches('/').to_string();
        self
    }

    /// Get the HTTP base URL.
    #[must_use]
    pub fn http_base(&self) -> &str {
        &self.http_base
    }

    /// Build a full URL for an API endpoint.
    #[must_use]
    pub fn url(&self, path: &str) -> String {
        format!("{}{}", self.http_base, path)
    }

    /// Build HTTP headers for API requests.
    #[cfg(feature = "http")]
    pub fn headers(&self) -> crate::error::Result<reqwest::header::HeaderMap> {
        use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let auth_value = format!("Bearer {}", self.api_key.expose_secret());
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&auth_value).map_err(|e| {
                crate::error::DoubaoError::Config(format!("Invalid API key format: {}", e))
            })?,
        );

        Ok(headers)
    }
}
