use std::sync::Arc;

use domain::attachment::PhotoId;
use domain::ports::MediaStoragePort;

use crate::errors::ApplicationError;

pub struct Deps {
    pub storage: Arc<dyn MediaStoragePort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(photo_id: PhotoId, deps: &Deps) -> Result<(), ApplicationError> {
    deps.storage.delete_photo(&photo_id).await?;
    Ok(())
}
