use std::sync::Arc;

use domain::errors::DomainError;
use domain::ports::{
    CascadeDeletePort, EventPublisherPort, MediaOwnershipPort, MediaStoragePort, UserQueryPort,
};
use domain::user::UserId;

use crate::errors::ApplicationError;
use crate::media::cleanup::delete_everything_owned_by;

pub struct Deps {
    pub user_query: Arc<dyn UserQueryPort>,
    pub cascade: Arc<dyn CascadeDeletePort>,
    pub media_storage: Arc<dyn MediaStoragePort>,
    pub media_ownership: Arc<dyn MediaOwnershipPort>,
    pub events: Arc<dyn EventPublisherPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(user_id: UserId, deps: &Deps) -> Result<(), ApplicationError> {
    deps.user_query
        .find_by_id(&user_id)
        .await?
        .ok_or_else(|| DomainError::NotFound("user not found".into()))?;

    let blobs =
        delete_everything_owned_by(&user_id, &deps.media_storage, &deps.media_ownership).await?;

    deps.cascade.delete_user_account(&user_id).await?;

    tracing::info!(%user_id, blobs, "user account deleted");

    Ok(())
}
