use std::sync::Arc;

use domain::entry::MoodEntry;
use domain::events::{DomainEvent, EventEnvelope};
use domain::ports::{
    ActivityQueryPort, EntryDimensionPort, EventPublisherPort, MoodEntryCommandPort,
};
use domain::user::UserId;

use crate::entry::tagging::verify_activities_may_be_tagged;
use crate::errors::ApplicationError;

use super::super::commands::CreateEntryCommand;

const NOTHING_YET: &[domain::activity::ActivityId] = &[];

pub struct Deps {
    pub entries: Arc<dyn MoodEntryCommandPort>,
    pub dimensions: Vec<Arc<dyn EntryDimensionPort>>,
    pub activities: Arc<dyn ActivityQueryPort>,
    pub events: Arc<dyn EventPublisherPort>,
}

#[tracing::instrument(skip(deps, wanted), fields(count = wanted.len()))]
pub async fn execute(
    user_id: UserId,
    wanted: Vec<CreateEntryCommand>,
    deps: &Deps,
) -> Result<Vec<MoodEntry>, ApplicationError> {
    for one in &wanted {
        verify_activities_may_be_tagged(&user_id, &one.dimensions, NOTHING_YET, &deps.activities)
            .await?;
    }

    let written: Vec<(MoodEntry, Vec<domain::dimension::DimensionValue>)> = wanted
        .into_iter()
        .map(|one| {
            (
                MoodEntry::new(one.user_id, one.mood, one.logged_at),
                one.dimensions,
            )
        })
        .collect();

    let entries: Vec<MoodEntry> = written.iter().map(|(entry, _)| entry.clone()).collect();
    deps.entries.save_batch(&entries).await?;

    for (entry, dimensions) in &written {
        for port in &deps.dimensions {
            port.save(entry.id(), dimensions).await?;
        }
    }

    for entry in &entries {
        let event = DomainEvent::EntryCreated {
            entry_id: entry.id().clone(),
            user_id: entry.user_id().clone(),
        };
        deps.events.publish(EventEnvelope::wrap(event)).await?;
    }

    tracing::info!(created = entries.len(), "created entries in one request");

    Ok(entries)
}
