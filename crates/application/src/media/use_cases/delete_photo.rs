use std::sync::Arc;

use domain::attachment::PhotoId;
use domain::ports::{MediaOwnershipPort, MediaStoragePort};
use domain::user::UserId;

use crate::errors::ApplicationError;
use crate::media::custody::verify_custody;

pub struct Deps {
    pub storage: Arc<dyn MediaStoragePort>,
    pub ownership: Arc<dyn MediaOwnershipPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    photo_id: PhotoId,
    caller_id: UserId,
    deps: &Deps,
) -> Result<(), ApplicationError> {
    let media = (&photo_id).into();
    verify_custody(media, &caller_id, &deps.ownership).await?;

    deps.storage.delete_photo(&photo_id).await?;
    deps.ownership.forget(media).await?;

    Ok(())
}
