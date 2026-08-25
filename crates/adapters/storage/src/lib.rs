mod media_storage;

pub use media_storage::ObjectStoreMediaStorage;

use config::{MediaBackend, StorageConfig};
use domain::errors::DomainError;

pub fn create_media_storage(
    config: &StorageConfig,
) -> Result<ObjectStoreMediaStorage, DomainError> {
    match &config.media {
        MediaBackend::Local { media_dir } => {
            let base = std::path::Path::new(&config.data_dir).join(media_dir);
            ObjectStoreMediaStorage::local(base)
        }
        MediaBackend::S3 {
            bucket,
            region,
            endpoint,
            access_key,
            secret_key,
        } => ObjectStoreMediaStorage::s3(
            bucket,
            region,
            endpoint.as_deref(),
            access_key.as_deref(),
            secret_key.as_deref(),
        ),
    }
}
