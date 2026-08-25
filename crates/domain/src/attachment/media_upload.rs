use crate::errors::DomainError;

use super::ContentType;

#[derive(Debug, Clone)]
pub struct MediaUpload {
    data: Vec<u8>,
    content_type: ContentType,
}

impl MediaUpload {
    pub fn new(data: Vec<u8>, content_type: ContentType) -> Result<Self, DomainError> {
        if data.is_empty() {
            return Err(DomainError::InvalidInput(
                "media data cannot be empty".into(),
            ));
        }
        Ok(Self { data, content_type })
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn content_type(&self) -> &ContentType {
        &self.content_type
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}
