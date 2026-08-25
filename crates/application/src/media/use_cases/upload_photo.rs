use std::sync::Arc;

use domain::attachment::{MediaUpload, PhotoId};
use domain::ports::MediaStoragePort;

use crate::errors::ApplicationError;

pub struct Deps {
    pub storage: Arc<dyn MediaStoragePort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(upload: MediaUpload, deps: &Deps) -> Result<PhotoId, ApplicationError> {
    let photo_id = deps.storage.store_photo(upload).await?;
    Ok(photo_id)
}
