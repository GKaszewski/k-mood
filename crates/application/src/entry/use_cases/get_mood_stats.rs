use std::sync::Arc;

use domain::entry::Mood;
use domain::ports::MoodEntryQueryPort;
use domain::services::MoodAnalyzerService;

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

    Ok(MoodStats {
        average: MoodAnalyzerService::average_mood(&entries),
        frequency: MoodAnalyzerService::mood_frequency(&entries),
        current_streak: MoodAnalyzerService::current_streak(&entries),
        total_entries: entries.len(),
    })
}
