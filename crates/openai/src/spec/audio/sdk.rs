#![cfg(not(target_family = "wasm"))]

use std::path::Path;

use crate::error::OpenAIError;
use crate::spec::audio::CreateSpeechResponse;
use crate::util::create_all_dir;

impl CreateSpeechResponse {
    pub async fn save<P: AsRef<Path>>(&self, file_path: P) -> Result<(), OpenAIError> {
        let dir = file_path.as_ref().parent();

        if let Some(dir) = dir {
            create_all_dir(dir)?;
        }

        tokio::fs::write(file_path, &self.bytes)
            .await
            .map_err(|e| OpenAIError::FileSave(e.to_string()))?;

        Ok(())
    }
}
