use std::sync::Arc;

use domain::entry::MoodEntry;
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
) -> Result<Vec<MoodEntry>, ApplicationError> {
    let entries = match query.range {
        Some(range) => {
            deps.query
                .find_by_date_range(&query.user_id, &range)
                .await?
        }
        None => {
            deps.query
                .find_by_user(&query.user_id, query.limit, query.offset)
                .await?
        }
    };

    Ok(entries)
}
