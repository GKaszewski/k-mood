use std::collections::HashMap;
use std::sync::Arc;

use crate::dimension::{DimensionKind, DimensionValue, lookup};
use crate::entry::MoodEntryId;
use crate::errors::DomainError;

use super::store::InMemoryStore;

pub struct InMemoryActivityDimension {
    store: Arc<InMemoryStore>,
}

impl InMemoryActivityDimension {
    pub fn sharing(store: Arc<InMemoryStore>) -> Self {
        Self { store }
    }
}

#[async_trait::async_trait]
impl crate::ports::EntryDimensionPort for InMemoryActivityDimension {
    async fn load(
        &self,
        entry_ids: &[MoodEntryId],
    ) -> Result<HashMap<MoodEntryId, DimensionValue>, DomainError> {
        let tagged = self.store.entry_activities.read().unwrap();

        Ok(entry_ids
            .iter()
            .filter_map(|id| tagged.get(id).map(|ids| (id.clone(), ids.clone())))
            .filter(|(_, ids)| !ids.is_empty())
            .map(|(id, ids)| (id, DimensionValue::activities(ids)))
            .collect())
    }

    async fn save(
        &self,
        entry_id: &MoodEntryId,
        values: &[DimensionValue],
    ) -> Result<(), DomainError> {
        let mut tagged = self.store.entry_activities.write().unwrap();

        let names_activities = values
            .iter()
            .any(|value| value.kind() == DimensionKind::Activities);

        match names_activities {
            true => tagged.insert(entry_id.clone(), lookup::activities_in(values).to_vec()),
            false => tagged.remove(entry_id),
        };

        Ok(())
    }
}
