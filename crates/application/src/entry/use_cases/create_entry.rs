use std::sync::Arc;

use domain::entry::MoodEntry;
use domain::events::{DomainEvent, EventEnvelope};
use domain::ports::{EntryDimensionPort, EventPublisherPort, MoodEntryCommandPort};

use crate::errors::ApplicationError;

use super::super::commands::CreateEntryCommand;

pub struct Deps {
    pub entries: Arc<dyn MoodEntryCommandPort>,
    pub dimensions: Vec<Arc<dyn EntryDimensionPort>>,
    pub events: Arc<dyn EventPublisherPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(cmd: CreateEntryCommand, deps: &Deps) -> Result<MoodEntry, ApplicationError> {
    let entry = MoodEntry::new(cmd.user_id, cmd.mood, cmd.logged_at);

    deps.entries.save(&entry).await?;
    for port in &deps.dimensions {
        port.save(entry.id(), &cmd.dimensions).await?;
    }

    let event = DomainEvent::EntryCreated {
        entry_id: entry.id().clone(),
        user_id: entry.user_id().clone(),
    };
    deps.events.publish(EventEnvelope::wrap(event)).await?;

    Ok(entry)
}
