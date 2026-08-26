use std::collections::HashMap;
use std::sync::RwLock;

use crate::dimension::{DimensionKind, DimensionValue};
use crate::entry::MoodEntryId;
use crate::errors::DomainError;

pub struct InMemoryDimensionStore {
    kind: DimensionKind,
    values: RwLock<HashMap<MoodEntryId, DimensionValue>>,
}

impl InMemoryDimensionStore {
    pub fn new(kind: DimensionKind) -> Self {
        Self {
            kind,
            values: RwLock::new(HashMap::new()),
        }
    }

    pub fn forget(&self, entry_id: &MoodEntryId) {
        self.values.write().unwrap().remove(entry_id);
    }

    pub fn holds_count(&self) -> usize {
        self.values.read().unwrap().len()
    }

    pub fn holds(&self, entry_id: &MoodEntryId) -> bool {
        self.values.read().unwrap().contains_key(entry_id)
    }

    pub fn put(&self, entry_id: &MoodEntryId, value: DimensionValue) {
        self.values.write().unwrap().insert(entry_id.clone(), value);
    }
}

#[async_trait::async_trait]
impl crate::ports::EntryDimensionPort for InMemoryDimensionStore {
    async fn load(
        &self,
        entry_ids: &[MoodEntryId],
    ) -> Result<HashMap<MoodEntryId, DimensionValue>, DomainError> {
        let stored = self.values.read().unwrap();
        Ok(entry_ids
            .iter()
            .filter_map(|id| stored.get(id).map(|value| (id.clone(), value.clone())))
            .collect())
    }

    async fn save(
        &self,
        entry_id: &MoodEntryId,
        values: &[DimensionValue],
    ) -> Result<(), DomainError> {
        let mut stored = self.values.write().unwrap();
        match values.iter().find(|value| value.kind() == self.kind) {
            Some(value) => stored.insert(entry_id.clone(), value.clone()),
            None => stored.remove(entry_id),
        };
        Ok(())
    }
}
