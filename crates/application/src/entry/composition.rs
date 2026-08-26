use std::collections::HashMap;
use std::sync::Arc;

use domain::dimension::{ComposedEntry, DimensionValue};
use domain::entry::{MoodEntry, MoodEntryId};
use domain::ports::EntryDimensionPort;

use crate::errors::ApplicationError;

pub struct EntryComposer {
    dimensions: Vec<Arc<dyn EntryDimensionPort>>,
}

impl EntryComposer {
    pub fn new(dimensions: Vec<Arc<dyn EntryDimensionPort>>) -> Self {
        Self { dimensions }
    }
}

impl EntryComposer {
    pub async fn compose(
        &self,
        entries: Vec<MoodEntry>,
    ) -> Result<Vec<ComposedEntry>, ApplicationError> {
        let entry_ids: Vec<MoodEntryId> = entries.iter().map(|e| e.id().clone()).collect();

        let mut loaded: Vec<HashMap<MoodEntryId, DimensionValue>> =
            Vec::with_capacity(self.dimensions.len());
        for port in &self.dimensions {
            loaded.push(port.load(&entry_ids).await?);
        }

        Ok(entries
            .into_iter()
            .map(|entry| {
                let dimensions = loaded
                    .iter()
                    .filter_map(|by_entry| by_entry.get(entry.id()).cloned())
                    .collect();
                ComposedEntry { entry, dimensions }
            })
            .collect())
    }
}
