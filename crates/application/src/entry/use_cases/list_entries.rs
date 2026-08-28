use std::sync::Arc;

use domain::entry::{MoodEntry, Page};
use domain::ports::MoodEntryQueryPort;

use crate::errors::ApplicationError;

use super::super::queries::ListEntriesQuery;

pub struct Deps {
    pub query: Arc<dyn MoodEntryQueryPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    query: ListEntriesQuery,
    deps: &Deps,
) -> Result<Page<MoodEntry>, ApplicationError> {
    let total = deps.query.count(&query.selection).await?;
    let entries = deps.query.select(&query.selection, query.page).await?;

    Ok(Page::new(entries, total, query.page))
}
