use std::sync::Arc;

use domain::entry::MoodEntryId;
use domain::errors::DomainError;
use domain::events::{DomainEvent, EventEnvelope};
use domain::ports::{
    EventPublisherPort, MediaStoragePort, MoodEntryCommandPort, MoodEntryQueryPort,
};
use domain::user::UserId;

use crate::authorization::verify_ownership;
use crate::errors::ApplicationError;

pub struct Deps {
    pub command: Arc<dyn MoodEntryCommandPort>,
    pub query: Arc<dyn MoodEntryQueryPort>,
    pub events: Arc<dyn EventPublisherPort>,
    pub media_storage: Arc<dyn MediaStoragePort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    entry_id: MoodEntryId,
    caller_id: UserId,
    deps: &Deps,
) -> Result<(), ApplicationError> {
    let entry = deps
        .query
        .find_by_id(&entry_id)
        .await?
        .ok_or_else(|| DomainError::NotFound("mood entry not found".into()))?;

    verify_ownership(entry.user_id(), &caller_id)?;

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

    deps.command.delete(&entry_id).await?;

    let event = DomainEvent::EntryDeleted {
        entry_id,
        user_id: entry.user_id().clone(),
    };
    deps.events.publish(EventEnvelope::wrap(event)).await?;

    Ok(())
}
