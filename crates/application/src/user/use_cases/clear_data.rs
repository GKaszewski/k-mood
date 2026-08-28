use std::sync::Arc;

use domain::ports::{CascadeDeletePort, MediaOwnershipPort, MediaStoragePort};
use domain::user::UserId;

use crate::errors::ApplicationError;
use crate::media::cleanup::delete_everything_owned_by;

pub struct Deps {
    pub cascade: Arc<dyn CascadeDeletePort>,
    pub media_storage: Arc<dyn MediaStoragePort>,
    pub media_ownership: Arc<dyn MediaOwnershipPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(user_id: UserId, deps: &Deps) -> Result<(), ApplicationError> {
    let blobs =
        delete_everything_owned_by(&user_id, &deps.media_storage, &deps.media_ownership).await?;

    deps.cascade.delete_all_user_data(&user_id).await?;

    tracing::info!(%user_id, blobs, "all user data cleared");

    Ok(())
}
