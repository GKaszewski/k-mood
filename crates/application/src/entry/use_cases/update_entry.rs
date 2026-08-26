use std::sync::Arc;

use domain::dimension::lookup;
use domain::entry::MoodEntry;
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

use super::super::commands::UpdateEntryCommand;

pub struct Deps {
    pub command: Arc<dyn MoodEntryCommandPort>,
    pub dimensions: Vec<Arc<dyn EntryDimensionPort>>,
    pub query: Arc<dyn MoodEntryQueryPort>,
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

    let previous = EntryComposer::new(deps.dimensions.clone())
        .compose(vec![entry.clone()])
        .await?;
    let old_photos = previous[0].photos().to_vec();
    let old_memos = previous[0].voice_memos().to_vec();

    entry.update_mood(cmd.mood);
    if let Some(logged_at) = cmd.logged_at {
        entry.update_logged_at(logged_at);
    }

    let new_photos = lookup::photos_in(&cmd.dimensions);
    let new_memos = lookup::voice_memos_in(&cmd.dimensions);

    for photo_id in &old_photos {
        if !new_photos.contains(photo_id)
            && let Err(e) = deps.media_storage.delete_photo(photo_id).await
        {
            tracing::warn!(%e, "failed to delete removed photo blob");
        }
    }
    for memo_id in &old_memos {
        if !new_memos.contains(memo_id)
            && let Err(e) = deps.media_storage.delete_voice_memo(memo_id).await
        {
            tracing::warn!(%e, "failed to delete removed voice memo blob");
        }
    }

    deps.command.save(&entry).await?;
    for port in &deps.dimensions {
        port.save(entry.id(), &cmd.dimensions).await?;
    }

    let event = DomainEvent::EntryUpdated {
        entry_id: entry.id().clone(),
        user_id: entry.user_id().clone(),
    };
    deps.events.publish(EventEnvelope::wrap(event)).await?;

    Ok(entry)
}
