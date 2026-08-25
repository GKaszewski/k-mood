use std::sync::Arc;

use domain::activity::ActivityId;
use domain::entry::DateRange;
use domain::ports::MoodEntryQueryPort;
use domain::services::MoodAnalyzerService;
use domain::user::UserId;

use crate::errors::ApplicationError;

pub struct Deps {
    pub query: Arc<dyn MoodEntryQueryPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    user_id: UserId,
    activity_id: ActivityId,
    range: Option<DateRange>,
    deps: &Deps,
) -> Result<Option<f64>, ApplicationError> {
    let entries = match range {
        Some(range) => deps.query.find_by_date_range(&user_id, &range).await?,
        None => deps.query.find_by_user(&user_id, None, None).await?,
    };

    Ok(MoodAnalyzerService::activity_mood_correlation(
        &entries,
        &activity_id,
    ))
}
