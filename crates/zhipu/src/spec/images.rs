//! Images request and response types.

use bytes::Bytes;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::ZhipuError;

/// Request to create images.
#[derive(Clone, Default, Debug, Builder, Serialize, Deserialize)]
#[builder(name = "CreateImageRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "ZhipuError"))]
pub struct CreateImageRequest {
    /// ID of the model to use.
    pub model: String,

    /// The prompt to generate images for.
    pub prompt: String,

    /// The size of the generated images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,

    /// User identifier for tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// An image object in the response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageData {
    /// The base64-encoded image data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b64_json: Option<String>,
    /// The URL of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Response from the images API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateImageResponse {
    /// Unix timestamp of creation.
    pub created: u64,
    /// List of generated images.
    pub data: Vec<ImageData>,
}

impl CreateImageResponse {
    /// Get the first image URL if available.
    #[must_use]
    pub fn first_url(&self) -> Option<&str> {
        self.data.first().and_then(|d| d.url.as_deref())
    }

    /// Get the first image as base64 if available.
    #[must_use]
    pub fn first_b64(&self) -> Option<&str> {
        self.data.first().and_then(|d| d.b64_json.as_deref())
    }
}

/// Image response with bytes.
#[derive(Debug, Clone)]
pub struct ImageBytes {
    /// The image data.
    pub bytes: Bytes,
}

impl ImageBytes {
    /// Save the image to a file.
    #[cfg(not(target_family = "wasm"))]
    pub async fn save<P: AsRef<std::path::Path>>(
        &self,
        path: P,
    ) -> Result<(), crate::error::ZhipuError> {
        tokio::fs::write(path, &self.bytes)
            .await
            .map_err(|e| crate::error::ZhipuError::FileError(e.to_string()))
    }
}

/// Available image generation models.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageModel {
    /// CogView-3 image generation model.
    CogView3,
    /// CogView-3 Plus model.
    CogView3Plus,
}

impl ImageModel {
    /// Get the model ID string.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CogView3 => "cogview-3",
            Self::CogView3Plus => "cogview-3-plus",
        }
    }
}

impl std::fmt::Display for ImageModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<ImageModel> for String {
    fn from(model: ImageModel) -> Self {
        model.as_str().to_string()
    }
}

/// Image size options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageSize {
    /// 1024x1024 pixels.
    Size1024x1024,
    /// 768x1344 pixels.
    Size768x1344,
    /// 864x1152 pixels.
    Size864x1152,
    /// 1344x768 pixels.
    Size1344x768,
    /// 1152x864 pixels.
    Size1152x864,
    /// 1440x720 pixels.
    Size1440x720,
    /// 720x1440 pixels.
    Size720x1440,
}

impl ImageSize {
    /// Get the size string.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Size1024x1024 => "1024x1024",
            Self::Size768x1344 => "768x1344",
            Self::Size864x1152 => "864x1152",
            Self::Size1344x768 => "1344x768",
            Self::Size1152x864 => "1152x864",
            Self::Size1440x720 => "1440x720",
            Self::Size720x1440 => "720x1440",
        }
    }
}

impl std::fmt::Display for ImageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<ImageSize> for String {
    fn from(size: ImageSize) -> Self {
        size.as_str().to_string()
    }
}
