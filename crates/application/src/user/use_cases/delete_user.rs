use std::sync::Arc;

use domain::errors::DomainError;
use domain::ports::{
    CascadeDeletePort, EventPublisherPort, MediaStoragePort, MoodEntryQueryPort, UserQueryPort,
};
use domain::user::UserId;

use crate::errors::ApplicationError;

pub struct Deps {
    pub user_query: Arc<dyn UserQueryPort>,
    pub entry_query: Arc<dyn MoodEntryQueryPort>,
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
    for entry in &entries {
        for photo_id in entry.photos() {
            if let Err(e) = deps.media_storage.delete_photo(photo_id).await {
                tracing::warn!(%e, "failed to delete photo blob");
            }
        }
        for memo_id in entry.voice_memos() {
            if let Err(e) = deps.media_storage.delete_voice_memo(memo_id).await {
                tracing::warn!(%e, "failed to delete voice memo blob");
            }
        }
    }

    deps.cascade.delete_user_account(&user_id).await?;

    tracing::info!(%user_id, "user account deleted");
    Ok(())
}
