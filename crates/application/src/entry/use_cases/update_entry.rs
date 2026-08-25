use std::sync::Arc;

use domain::entry::MoodEntry;
use domain::errors::DomainError;
use domain::events::{DomainEvent, EventEnvelope};
use domain::ports::{
    EventPublisherPort, MediaStoragePort, MoodEntryCommandPort, MoodEntryQueryPort,
};
use domain::user::UserId;

use crate::authorization::verify_ownership;
use crate::errors::ApplicationError;

use super::super::commands::UpdateEntryCommand;

pub struct Deps {
    pub command: Arc<dyn MoodEntryCommandPort>,
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

    let old_photos = entry.photos().to_vec();
    let old_memos = entry.voice_memos().to_vec();

    entry.update_mood(cmd.mood);
    entry.update_logged_at(cmd.logged_at);
    entry.set_content(cmd.content);
    entry.set_activities(cmd.activities);
    entry.set_photos(cmd.photos);
    entry.set_voice_memos(cmd.voice_memos);

    for photo_id in &old_photos {
        if !entry.photos().contains(photo_id)
            && let Err(e) = deps.media_storage.delete_photo(photo_id).await
        {
            tracing::warn!(%e, "failed to delete removed photo blob");
        }
    }
    for memo_id in &old_memos {
        if !entry.voice_memos().contains(memo_id)
            && let Err(e) = deps.media_storage.delete_voice_memo(memo_id).await
        {
            tracing::warn!(%e, "failed to delete removed voice memo blob");
        }
    }

    deps.command.save(&entry).await?;

    let event = DomainEvent::EntryUpdated {
        entry_id: entry.id().clone(),
        user_id: entry.user_id().clone(),
    };
    deps.events.publish(EventEnvelope::wrap(event)).await?;

    Ok(entry)
}
