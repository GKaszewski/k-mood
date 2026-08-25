use crate::attachment::{ContentType, MediaUpload, PhotoId, VoiceMemoId};
use crate::errors::DomainError;

pub struct MediaFile {
    pub data: Vec<u8>,
    pub content_type: ContentType,
}

#[async_trait::async_trait]
pub trait MediaStoragePort: Send + Sync {
    async fn store_photo(&self, upload: MediaUpload) -> Result<PhotoId, DomainError>;
    async fn store_voice_memo(&self, upload: MediaUpload) -> Result<VoiceMemoId, DomainError>;
    async fn get_photo(&self, id: &PhotoId) -> Result<Option<MediaFile>, DomainError>;
    async fn get_voice_memo(&self, id: &VoiceMemoId) -> Result<Option<MediaFile>, DomainError>;
    async fn delete_photo(&self, id: &PhotoId) -> Result<(), DomainError>;
    async fn delete_voice_memo(&self, id: &VoiceMemoId) -> Result<(), DomainError>;
}
