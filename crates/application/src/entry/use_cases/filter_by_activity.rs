use std::sync::Arc;

use domain::entry::MoodEntry;
use domain::ports::MoodEntryQueryPort;

use crate::errors::ApplicationError;

use super::super::queries::FilterByActivityQuery;

pub struct Deps {
    pub query: Arc<dyn MoodEntryQueryPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    query: FilterByActivityQuery,
    deps: &Deps,
) -> Result<Vec<MoodEntry>, ApplicationError> {
    let entries = deps
        .query
        .find_by_activity(&query.user_id, &query.activity_id)
        .await?;
    Ok(entries)
}
