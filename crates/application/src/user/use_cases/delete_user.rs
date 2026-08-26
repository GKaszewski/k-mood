use std::sync::Arc;

use domain::errors::DomainError;
use domain::ports::{
    CascadeDeletePort, EntryDimensionPort, EventPublisherPort, MediaStoragePort,
    MoodEntryQueryPort, UserQueryPort,
};
use domain::user::UserId;

use crate::entry::composition::EntryComposer;
use crate::errors::ApplicationError;
use crate::media::cleanup::delete_media_for;

pub struct Deps {
    pub user_query: Arc<dyn UserQueryPort>,
    pub entry_query: Arc<dyn MoodEntryQueryPort>,
    pub dimensions: Vec<Arc<dyn EntryDimensionPort>>,
    pub cascade: Arc<dyn CascadeDeletePort>,
    pub media_storage: Arc<dyn MediaStoragePort>,
    pub events: Arc<dyn EventPublisherPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(user_id: UserId, deps: &Deps) -> Result<(), ApplicationError> {
    deps.user_query
        .find_by_id(&user_id)
        .await?
        .ok_or_else(|| DomainError::NotFound("user not found".into()))?;

    let entries = deps.entry_query.find_by_user(&user_id, None, None).await?;
    let composed = EntryComposer::new(deps.dimensions.clone())
        .compose(entries)
        .await?;
    delete_media_for(&composed, &deps.media_storage).await;

    deps.cascade.delete_user_account(&user_id).await?;

    tracing::info!(%user_id, "user account deleted");
    Ok(())
}
