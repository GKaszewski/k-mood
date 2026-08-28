use std::sync::Arc;

use domain::dimension::DimensionValue;
use domain::entry::MoodEntry;
use domain::errors::DomainError;
use domain::events::{DomainEvent, EventEnvelope};
use domain::ports::{
    ActivityQueryPort, EntryDimensionPort, EventPublisherPort, MediaStoragePort,
    MoodEntryCommandPort, MoodEntryQueryPort,
};
use domain::user::UserId;

use crate::authorization::verify_ownership;
use crate::entry::composition::EntryComposer;
use crate::entry::tagging::verify_activities_may_be_tagged;
use crate::errors::ApplicationError;
use crate::media::dropped::{DroppedMedia, delete_dropped};

use super::super::commands::UpdateEntryCommand;

pub struct Deps {
    pub command: Arc<dyn MoodEntryCommandPort>,
    pub dimensions: Vec<Arc<dyn EntryDimensionPort>>,
    pub query: Arc<dyn MoodEntryQueryPort>,
    pub activities: Arc<dyn ActivityQueryPort>,
    pub media_storage: Arc<dyn MediaStoragePort>,
    pub events: Arc<dyn EventPublisherPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    cmd: UpdateEntryCommand,
    caller_id: UserId,
    deps: &Deps,
) -> Result<MoodEntry, ApplicationError> {
    let mut entry = deps
        .query
        .find_by_id(&cmd.entry_id)
        .await?
        .ok_or_else(|| DomainError::NotFound("mood entry not found".into()))?;

    verify_ownership(entry.user_id(), &caller_id)?;

    let dropped = match &cmd.dimensions {
        Some(wanted) => Some(replace_dimensions(&entry, wanted, &caller_id, deps).await?),
        None => None,
    };

    if let Some(mood) = cmd.mood {
        entry.update_mood(mood);
    }
    if let Some(logged_at) = cmd.logged_at {
        entry.update_logged_at(logged_at);
    }

    deps.command.save(&entry).await?;

    if let Some(wanted) = &cmd.dimensions {
        for port in &deps.dimensions {
            port.save(entry.id(), wanted).await?;
        }
    }

    if let Some(dropped) = dropped {
        delete_dropped(&dropped, &deps.media_storage).await;
    }

    let event = DomainEvent::EntryUpdated {
        entry_id: entry.id().clone(),
        user_id: entry.user_id().clone(),
    };
    deps.events.publish(EventEnvelope::wrap(event)).await?;

    Ok(entry)
}

async fn replace_dimensions(
    entry: &MoodEntry,
    wanted: &[DimensionValue],
    caller_id: &UserId,
    deps: &Deps,
) -> Result<DroppedMedia, ApplicationError> {
    let previous = EntryComposer::new(deps.dimensions.clone())
        .compose(vec![entry.clone()])
        .await?;

    verify_activities_may_be_tagged(
        caller_id,
        wanted,
        previous[0].activities(),
        &deps.activities,
    )
    .await?;

    Ok(DroppedMedia::between(&previous[0], wanted))
}
