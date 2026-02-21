use std::fmt::{self, Display};
use std::ops::Deref;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct TextObject {
    pub text: String,
}

impl From<&str> for TextObject {
    fn from(value: &str) -> Self {
        TextObject { text: value.into() }
    }
}

impl From<String> for TextObject {
    fn from(value: String) -> Self {
        TextObject { text: value }
    }
}

impl Display for TextObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Deref for TextObject {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.text
    }
}

impl AsRef<String> for TextObject {
    fn as_ref(&self) -> &String {
        &self.text
    }
}
impl AsRef<str> for TextObject {
    fn as_ref(&self) -> &str {
        &self.text
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum PartibleTextContent {
    /// The text contents of the message.
    Text(String),
    /// An array of content parts with a defined type. Supported options differ based on the [model](https://platform.openai.com/docs/models) being used to generate the response. Can contain text, image, or audio inputs.
    Array(Vec<TextObject>),
}
impl PartibleTextContent {
    pub fn to_texts(&self) -> Vec<String> {
        match self {
            Self::Text(text) => vec![text.clone()],
            Self::Array(parts) => parts.iter().map(|p| p.to_string()).collect(),
        }
    }
    pub fn into_texts(self) -> Vec<String> {
        match self {
            Self::Text(text) => vec![text],
            Self::Array(parts) => parts.into_iter().map(|p| p.to_string()).collect(),
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Text(text) => text.is_empty(),
            Self::Array(parts) => parts.is_empty(),
        }
    }
}
impl Display for PartibleTextContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(text) => write!(f, "{text}"),
            Self::Array(parts) => write!(
                f,
                "{}",
                parts
                    .iter()
                    .map(|p| &*p.text)
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
        }
    }
}
impl From<&str> for PartibleTextContent {
    fn from(value: &str) -> Self {
        Self::Text(value.into())
    }
}

impl From<String> for PartibleTextContent {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}
impl Default for PartibleTextContent {
    fn default() -> Self {
        Self::Text("".into())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum UnitaryTextContent {
    Text(TextObject),
}
impl Display for UnitaryTextContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(text) => write!(f, "{text}"),
        }
    }
}
impl From<&str> for UnitaryTextContent {
    fn from(value: &str) -> Self {
        Self::Text(value.into())
    }
}

impl From<String> for UnitaryTextContent {
    fn from(value: String) -> Self {
        Self::Text(value.into())
    }
}
impl From<TextObject> for UnitaryTextContent {
    fn from(value: TextObject) -> Self {
        Self::Text(value)
    }
}
impl Default for UnitaryTextContent {
    fn default() -> Self {
        Self::Text("".into())
    }
}
