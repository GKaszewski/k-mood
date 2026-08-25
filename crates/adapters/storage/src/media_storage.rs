use std::sync::Arc;

use bytes::Bytes;
use object_store::aws::AmazonS3Builder;
use object_store::local::LocalFileSystem;
use object_store::path::Path;
use object_store::{GetOptions, ObjectStore, PutOptions, PutPayload};

use domain::attachment::{ContentType, MediaUpload, PhotoId, VoiceMemoId};
use domain::errors::DomainError;
use domain::ports::MediaFile;

pub struct ObjectStoreMediaStorage {
    store: Arc<dyn ObjectStore>,
}

impl ObjectStoreMediaStorage {
    pub fn local(base_path: std::path::PathBuf) -> Result<Self, DomainError> {
        std::fs::create_dir_all(&base_path).map_err(|e| {
            DomainError::InvalidInput(format!("failed to create media directory: {e}"))
        })?;

        let store = LocalFileSystem::new_with_prefix(base_path)
            .map_err(|e| DomainError::InvalidInput(format!("failed to init local storage: {e}")))?;

        Ok(Self {
            store: Arc::new(store),
        })
    }

    pub fn s3(
        bucket: &str,
        region: &str,
        endpoint: Option<&str>,
        access_key: Option<&str>,
        secret_key: Option<&str>,
    ) -> Result<Self, DomainError> {
        let mut builder = AmazonS3Builder::new()
            .with_bucket_name(bucket)
            .with_region(region);

        if let Some(endpoint) = endpoint {
            builder = builder
                .with_endpoint(endpoint)
                .with_virtual_hosted_style_request(false);
        }
        if let Some(key) = access_key {
            builder = builder.with_access_key_id(key);
        }
        if let Some(secret) = secret_key {
            builder = builder.with_secret_access_key(secret);
        }

        let store = builder
            .build()
            .map_err(|e| DomainError::InvalidInput(format!("failed to init S3 storage: {e}")))?;

        Ok(Self {
            store: Arc::new(store),
        })
    }

    async fn store_blob(
        &self,
        prefix: &str,
        data: &[u8],
        content_type: &str,
    ) -> Result<uuid::Uuid, DomainError> {
        let id = uuid::Uuid::new_v4();

        let blob_path = Path::from(format!("{prefix}/{id}"));
        let payload = PutPayload::from(Bytes::copy_from_slice(data));
        self.store
            .put_opts(&blob_path, payload, PutOptions::default())
            .await
            .map_err(|e| DomainError::InvalidInput(format!("failed to store media: {e}")))?;

        let meta_path = Path::from(format!("{prefix}/{id}.meta"));
        let meta_payload = PutPayload::from(Bytes::from(content_type.to_string()));
        self.store
            .put_opts(&meta_path, meta_payload, PutOptions::default())
            .await
            .map_err(|e| DomainError::InvalidInput(format!("failed to store metadata: {e}")))?;

        Ok(id)
    }

    async fn get_blob(
        &self,
        prefix: &str,
        id: uuid::Uuid,
    ) -> Result<Option<MediaFile>, DomainError> {
        let blob_path = Path::from(format!("{prefix}/{id}"));
        let data = match self.store.get_opts(&blob_path, GetOptions::default()).await {
            Ok(result) => result
                .bytes()
                .await
                .map_err(|e| DomainError::InvalidInput(format!("failed to read media: {e}")))?
                .to_vec(),
            Err(object_store::Error::NotFound { .. }) => return Ok(None),
            Err(e) => {
                return Err(DomainError::InvalidInput(format!(
                    "failed to get media: {e}"
                )));
            }
        };

        let meta_path = Path::from(format!("{prefix}/{id}.meta"));
        let content_type = match self.store.get_opts(&meta_path, GetOptions::default()).await {
            Ok(result) => {
                let bytes = result.bytes().await.unwrap_or_default();
                let ct_str = String::from_utf8(bytes.to_vec()).unwrap_or_default();
                ContentType::from_persistence(ct_str)
            }
            _ => ContentType::from_persistence("application/octet-stream".into()),
        };

        Ok(Some(MediaFile { data, content_type }))
    }

    async fn remove_blob(&self, prefix: &str, id: uuid::Uuid) -> Result<(), DomainError> {
        let blob_path = Path::from(format!("{prefix}/{id}"));
        let meta_path = Path::from(format!("{prefix}/{id}.meta"));

        let stream = futures_util::stream::iter(vec![Ok(blob_path), Ok(meta_path)]);
        let results: Vec<_> = self.store.delete_stream(Box::pin(stream)).collect().await;

        for result in results {
            match result {
                Err(object_store::Error::NotFound { .. }) => {}
                Err(e) => {
                    return Err(DomainError::InvalidInput(format!(
                        "failed to delete media: {e}"
                    )));
                }
                Ok(_) => {}
            }
        }

        Ok(())
    }
}

use futures_util::StreamExt;

#[async_trait::async_trait]
impl domain::ports::MediaStoragePort for ObjectStoreMediaStorage {
    async fn store_photo(&self, upload: MediaUpload) -> Result<PhotoId, DomainError> {
        let id = self
            .store_blob("photos", upload.data(), upload.content_type().value())
            .await?;
        Ok(PhotoId::from_uuid(id))
    }

    async fn store_voice_memo(&self, upload: MediaUpload) -> Result<VoiceMemoId, DomainError> {
        let id = self
            .store_blob("voice_memos", upload.data(), upload.content_type().value())
            .await?;
        Ok(VoiceMemoId::from_uuid(id))
    }

    async fn get_photo(&self, id: &PhotoId) -> Result<Option<MediaFile>, DomainError> {
        self.get_blob("photos", id.value()).await
    }

    async fn get_voice_memo(&self, id: &VoiceMemoId) -> Result<Option<MediaFile>, DomainError> {
        self.get_blob("voice_memos", id.value()).await
    }

    async fn delete_photo(&self, id: &PhotoId) -> Result<(), DomainError> {
        self.remove_blob("photos", id.value()).await
    }

    async fn delete_voice_memo(&self, id: &VoiceMemoId) -> Result<(), DomainError> {
        self.remove_blob("voice_memos", id.value()).await
    }
}
