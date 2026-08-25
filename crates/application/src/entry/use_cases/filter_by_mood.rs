use std::sync::Arc;

use domain::entry::MoodEntry;
use domain::ports::MoodEntryQueryPort;

use crate::errors::ApplicationError;

use super::super::queries::FilterByMoodQuery;

pub struct Deps {
    pub query: Arc<dyn MoodEntryQueryPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    query: FilterByMoodQuery,
    deps: &Deps,
) -> Result<Vec<MoodEntry>, ApplicationError> {
    let entries = deps.query.find_by_mood(&query.user_id, query.mood).await?;
    Ok(entries)
}
