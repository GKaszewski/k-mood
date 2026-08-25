use axum::extract::Multipart;

use domain::attachment::{ContentType, MediaUpload};

use crate::errors::ApiError;

pub async fn extract_media_upload(mut multipart: Multipart) -> Result<MediaUpload, ApiError> {
    let mut file_data: Option<Vec<u8>> = None;
    let mut content_type_str: Option<String> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| validation_error(format!("invalid multipart data: {e}")))?
    {
        match field.name() {
            Some("file") => {
                if content_type_str.is_none() {
                    content_type_str = field.content_type().map(|s| s.to_string());
                }
                let bytes = field
                    .bytes()
                    .await
                    .map_err(|e| validation_error(format!("failed to read file: {e}")))?;
                file_data = Some(bytes.to_vec());
            }
            Some("content_type") | Some("contentType") => {
                let text = field
                    .text()
                    .await
                    .map_err(|e| validation_error(format!("failed to read content type: {e}")))?;
                content_type_str = Some(text);
            }
            _ => {}
        }
    }

    let data = file_data.ok_or_else(|| validation_error("missing 'file' field".into()))?;
    let content_type_str =
        content_type_str.ok_or_else(|| validation_error("missing content type".into()))?;
    let content_type = ContentType::new(content_type_str)?;

    MediaUpload::new(data, content_type).map_err(Into::into)
}

pub async fn extract_file_bytes(mut multipart: Multipart) -> Result<Vec<u8>, ApiError> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| validation_error(format!("invalid multipart data: {e}")))?
    {
        if field.name() == Some("file") {
            let bytes = field
                .bytes()
                .await
                .map_err(|e| validation_error(format!("failed to read file: {e}")))?;
            return Ok(bytes.to_vec());
        }
    }

    Err(validation_error("missing 'file' field".into()))
}

fn validation_error(msg: String) -> ApiError {
    ApiError(application::errors::ApplicationError::Validation(msg))
}
