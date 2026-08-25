use std::sync::Arc;

use domain::entry::MoodEntry;
use domain::events::{DomainEvent, EventEnvelope};
use domain::ports::{EventPublisherPort, MoodEntryCommandPort};

use crate::errors::ApplicationError;

use super::super::commands::CreateEntryCommand;

pub struct Deps {
    pub entries: Arc<dyn MoodEntryCommandPort>,
    pub events: Arc<dyn EventPublisherPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(cmd: CreateEntryCommand, deps: &Deps) -> Result<MoodEntry, ApplicationError> {
    let mut entry = MoodEntry::new(cmd.user_id, cmd.mood, cmd.logged_at);

    entry.set_content(cmd.content);
    entry.set_activities(cmd.activities);
    entry.set_photos(cmd.photos);
    entry.set_voice_memos(cmd.voice_memos);

    deps.entries.save(&entry).await?;

    let event = DomainEvent::EntryCreated {
        entry_id: entry.id().clone(),
        user_id: entry.user_id().clone(),
    };
    deps.events.publish(EventEnvelope::wrap(event)).await?;

    Ok(entry)
}
