use std::collections::HashMap;
use std::sync::RwLock;

use crate::attachment::{ContentType, MediaUpload, PhotoId, VoiceMemoId};
use crate::errors::DomainError;
use crate::ports::MediaFile;

type Blob = (Vec<u8>, ContentType);

pub struct FakeMediaStorage {
    photos: RwLock<HashMap<PhotoId, Blob>>,
    voice_memos: RwLock<HashMap<VoiceMemoId, Blob>>,
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
        self.put_photo_as(
            id,
            bytes,
            ContentType::from_persistence("image/jpeg".into()),
        );
    }

    pub fn put_photo_as(&self, id: &PhotoId, bytes: &[u8], content_type: ContentType) {
        self.photos
            .write()
            .unwrap()
            .insert(id.clone(), (bytes.to_vec(), content_type));
    }

    pub fn put_voice_memo(&self, id: &VoiceMemoId, bytes: &[u8]) {
        self.put_voice_memo_as(
            id,
            bytes,
            ContentType::from_persistence("audio/webm".into()),
        );
    }

    pub fn put_voice_memo_as(&self, id: &VoiceMemoId, bytes: &[u8], content_type: ContentType) {
        self.voice_memos
            .write()
            .unwrap()
            .insert(id.clone(), (bytes.to_vec(), content_type));
    }

    pub fn type_of_photo(&self, id: &PhotoId) -> Option<ContentType> {
        self.photos
            .read()
            .unwrap()
            .get(id)
            .map(|(_, content_type)| content_type.clone())
    }

    pub fn type_of_voice_memo(&self, id: &VoiceMemoId) -> Option<ContentType> {
        self.voice_memos
            .read()
            .unwrap()
            .get(id)
            .map(|(_, content_type)| content_type.clone())
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
        self.put_photo_as(&id, upload.data(), upload.content_type().clone());

        Ok(id)
    }

    async fn store_voice_memo(&self, upload: MediaUpload) -> Result<VoiceMemoId, DomainError> {
        let id = VoiceMemoId::generate();
        self.put_voice_memo_as(&id, upload.data(), upload.content_type().clone());

        Ok(id)
    }

    async fn get_photo(&self, id: &PhotoId) -> Result<Option<MediaFile>, DomainError> {
        Ok(self
            .photos
            .read()
            .unwrap()
            .get(id)
            .map(|(data, content_type)| MediaFile {
                data: data.clone(),
                content_type: content_type.clone(),
            }))
    }

    async fn get_voice_memo(&self, id: &VoiceMemoId) -> Result<Option<MediaFile>, DomainError> {
        Ok(self
            .voice_memos
            .read()
            .unwrap()
            .get(id)
            .map(|(data, content_type)| MediaFile {
                data: data.clone(),
                content_type: content_type.clone(),
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
