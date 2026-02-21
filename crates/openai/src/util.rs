use std::path::Path;

use reqwest::Body;

use crate::error::OpenAIError;
use crate::spec::InputSource;

#[cfg(not(target_family = "wasm"))]
pub(crate) async fn file_stream_body(source: InputSource) -> Result<Body, OpenAIError> {
    let body = match source {
        InputSource::Path { path } => {
            let file = tokio::fs::File::open(path)
                .await
                .map_err(|e| OpenAIError::FileRead(e.to_string()))?;
            let stream =
                tokio_util::codec::FramedRead::new(file, tokio_util::codec::BytesCodec::new());
            Body::wrap_stream(stream)
        }
        _ => {
            return Err(OpenAIError::FileRead(
                "Cannot create stream from non-file source".to_string(),
            ));
        }
    };
    Ok(body)
}

/// Creates the part for the given file for multipart upload.
pub(crate) async fn create_file_part(
    source: InputSource,
) -> Result<reqwest::multipart::Part, OpenAIError> {
    let (stream, file_name) = match source {
        #[cfg(not(target_family = "wasm"))]
        InputSource::Path { path } => {
            let file_name = path
                .file_name()
                .ok_or_else(|| {
                    OpenAIError::FileRead(format!(
                        "cannot extract file name from {}",
                        path.display()
                    ))
                })?
                .to_str()
                .unwrap()
                .to_string();

            (
                file_stream_body(InputSource::Path { path }).await?,
                file_name,
            )
        }
        InputSource::Bytes { filename, bytes } => (Body::from(bytes), filename),
        InputSource::VecU8 { filename, vec } => (Body::from(vec), filename),
    };

    let file_part = reqwest::multipart::Part::stream(stream).file_name(file_name);

    Ok(file_part)
}

#[cfg(all(any(feature = "image", feature = "audio"), not(target_family = "wasm")))]
pub(crate) fn create_all_dir<P: AsRef<Path>>(dir: P) -> Result<(), OpenAIError> {
    let exists = match Path::try_exists(dir.as_ref()) {
        Ok(exists) => exists,
        Err(e) => return Err(OpenAIError::FileSave(e.to_string())),
    };

    if !exists {
        std::fs::create_dir_all(dir.as_ref()).map_err(|e| OpenAIError::FileSave(e.to_string()))?;
    }

    Ok(())
}
