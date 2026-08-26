use std::sync::Arc;

use domain::entry::MoodEntryId;
use domain::errors::DomainError;
use domain::events::{DomainEvent, EventEnvelope};
use domain::ports::{
    EntryDimensionPort, EventPublisherPort, MediaStoragePort, MoodEntryCommandPort,
    MoodEntryQueryPort,
};
use domain::user::UserId;

use crate::authorization::verify_ownership;
use crate::entry::composition::EntryComposer;
use crate::errors::ApplicationError;
use crate::media::cleanup::delete_media_for;

pub struct Deps {
    pub command: Arc<dyn MoodEntryCommandPort>,
    pub query: Arc<dyn MoodEntryQueryPort>,
    pub dimensions: Vec<Arc<dyn EntryDimensionPort>>,
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

    let user_id = entry.user_id().clone();
    let composed = EntryComposer::new(deps.dimensions.clone())
        .compose(vec![entry])
        .await?;
    delete_media_for(&composed, &deps.media_storage).await;

    deps.command.delete(&entry_id).await?;

    let event = DomainEvent::EntryDeleted { entry_id, user_id };
    deps.events.publish(EventEnvelope::wrap(event)).await?;

    Ok(())
}
