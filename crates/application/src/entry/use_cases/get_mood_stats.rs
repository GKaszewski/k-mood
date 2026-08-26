use std::sync::Arc;

use domain::entry::Mood;
use domain::ports::{MoodEntryQueryPort, UserQueryPort};
use domain::services::MoodAnalyzerService;

use crate::day::{date_of, timezone_for, today_in};
use crate::errors::ApplicationError;

use super::super::queries::MoodStatsQuery;

pub struct MoodStats {
    pub average: Option<f64>,
    pub frequency: Vec<(Mood, usize)>,
    pub current_streak: usize,
    pub total_entries: usize,
}

pub struct Deps {
    pub query: Arc<dyn MoodEntryQueryPort>,
    pub users: Arc<dyn UserQueryPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(query: MoodStatsQuery, deps: &Deps) -> Result<MoodStats, ApplicationError> {
    let entries = match query.range {
        Some(range) => {
            deps.query
                .find_by_date_range(&query.user_id, &range)
                .await?
        }
        None => deps.query.find_by_user(&query.user_id, None, None).await?,
    };

    let timezone = timezone_for(&query.user_id, &deps.users).await?;
    let dates: Vec<_> = entries
        .iter()
        .map(|entry| date_of(entry, &timezone))
        .collect();

    Ok(MoodStats {
        average: MoodAnalyzerService::average_mood(&entries),
        frequency: MoodAnalyzerService::mood_frequency(&entries),
        current_streak: MoodAnalyzerService::current_streak(&dates, today_in(&timezone)),
        total_entries: entries.len(),
    })
}
