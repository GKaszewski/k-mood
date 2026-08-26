use std::collections::HashMap;
use std::sync::RwLock;

use crate::attachment::{ContentType, MediaUpload, PhotoId, VoiceMemoId};
use crate::errors::DomainError;
use crate::ports::MediaFile;

pub struct FakeMediaStorage {
    photos: RwLock<HashMap<PhotoId, Vec<u8>>>,
    voice_memos: RwLock<HashMap<VoiceMemoId, Vec<u8>>>,
    deletions_attempted: RwLock<Vec<String>>,
    refusing_to_delete: bool,
}

impl FakeMediaStorage {
    pub fn holding_nothing() -> Self {
        Self {
            photos: RwLock::new(HashMap::new()),
            voice_memos: RwLock::new(HashMap::new()),
            deletions_attempted: RwLock::new(Vec::new()),
            refusing_to_delete: false,
        }
    }

    pub fn refusing_to_delete() -> Self {
        Self {
            refusing_to_delete: true,
            ..Self::holding_nothing()
        }
    }

    pub fn put_photo(&self, id: &PhotoId, bytes: &[u8]) {
        self.photos
            .write()
            .unwrap()
            .insert(id.clone(), bytes.to_vec());
    }

    pub fn put_voice_memo(&self, id: &VoiceMemoId, bytes: &[u8]) {
        self.voice_memos
            .write()
            .unwrap()
            .insert(id.clone(), bytes.to_vec());
    }

    pub fn holds_photo(&self, id: &PhotoId) -> bool {
        self.photos.read().unwrap().contains_key(id)
    }

    pub fn holds_voice_memo(&self, id: &VoiceMemoId) -> bool {
        self.voice_memos.read().unwrap().contains_key(id)
    }

    pub fn blobs_held(&self) -> usize {
        self.photos.read().unwrap().len() + self.voice_memos.read().unwrap().len()
    }

    pub fn deletions_attempted(&self) -> Vec<String> {
        self.deletions_attempted.read().unwrap().clone()
    }
}

impl Default for FakeMediaStorage {
    fn default() -> Self {
        Self::holding_nothing()
    }
}

#[async_trait::async_trait]
impl crate::ports::MediaStoragePort for FakeMediaStorage {
    async fn store_photo(&self, upload: MediaUpload) -> Result<PhotoId, DomainError> {
        let id = PhotoId::generate();
        self.put_photo(&id, upload.data());

        Ok(id)
    }

    async fn store_voice_memo(&self, upload: MediaUpload) -> Result<VoiceMemoId, DomainError> {
        let id = VoiceMemoId::generate();
        self.put_voice_memo(&id, upload.data());

        Ok(id)
    }

    async fn get_photo(&self, id: &PhotoId) -> Result<Option<MediaFile>, DomainError> {
        Ok(self.photos.read().unwrap().get(id).map(|bytes| MediaFile {
            data: bytes.clone(),
            content_type: ContentType::from_persistence("image/jpeg".into()),
        }))
    }

    async fn get_voice_memo(&self, id: &VoiceMemoId) -> Result<Option<MediaFile>, DomainError> {
        Ok(self
            .voice_memos
            .read()
            .unwrap()
            .get(id)
            .map(|bytes| MediaFile {
                data: bytes.clone(),
                content_type: ContentType::from_persistence("audio/webm".into()),
            }))
    }

    async fn delete_photo(&self, id: &PhotoId) -> Result<(), DomainError> {
        self.deletions_attempted
            .write()
            .unwrap()
            .push(id.value().to_string());

        if self.refusing_to_delete {
            return Err(DomainError::InvalidInput(
                "the blob store refused to delete this photo".into(),
            ));
        }

        self.photos.write().unwrap().remove(id);

        Ok(())
    }

    async fn delete_voice_memo(&self, id: &VoiceMemoId) -> Result<(), DomainError> {
        self.deletions_attempted
            .write()
            .unwrap()
            .push(id.value().to_string());

        if self.refusing_to_delete {
            return Err(DomainError::InvalidInput(
                "the blob store refused to delete this voice memo".into(),
            ));
        }

        self.voice_memos.write().unwrap().remove(id);

        Ok(())
    }
}
