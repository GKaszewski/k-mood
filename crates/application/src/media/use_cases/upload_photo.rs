use std::sync::Arc;

use domain::attachment::{MediaUpload, PhotoId};
use domain::ports::{MediaOwnershipPort, MediaStoragePort};
use domain::user::UserId;

use crate::errors::ApplicationError;

pub struct Deps {
    pub storage: Arc<dyn MediaStoragePort>,
    pub ownership: Arc<dyn MediaOwnershipPort>,
}

#[tracing::instrument(skip(deps, upload))]
pub async fn execute(
    owner: UserId,
    upload: MediaUpload,
    deps: &Deps,
) -> Result<PhotoId, ApplicationError> {
    let photo_id = deps.storage.store_photo(upload).await?;

    deps.ownership.remember(&owner, (&photo_id).into()).await?;

    Ok(photo_id)
}
