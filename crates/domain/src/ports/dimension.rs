use std::collections::HashMap;

use crate::dimension::DimensionValue;
use crate::entry::MoodEntryId;
use crate::errors::DomainError;

#[async_trait::async_trait]
pub trait EntryDimensionPort: Send + Sync {
    async fn load(
        &self,
        entry_ids: &[MoodEntryId],
    ) -> Result<HashMap<MoodEntryId, DimensionValue>, DomainError>;

    async fn save(
        &self,
        entry_id: &MoodEntryId,
        values: &[DimensionValue],
    ) -> Result<(), DomainError>;
}
