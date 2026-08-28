use application::entry::use_cases::get_mood_stats::MoodStats;

use crate::responses::{MoodFrequencyResponse, MoodStatsResponse};

impl From<MoodStats> for MoodStatsResponse {
    fn from(stats: MoodStats) -> Self {
        Self {
            average: stats.average,
            frequency: stats
                .frequency
                .into_iter()
                .map(|(mood, count)| MoodFrequencyResponse {
                    mood: mood.value(),
                    mood_label: mood.label().to_string(),
                    count,
                })
                .collect(),
            current_streak: stats.current_streak,
            total_entries: stats.total_entries,
            first_logged_on: stats.first_logged_on.map(|date| date.to_string()),
            last_logged_on: stats.last_logged_on.map(|date| date.to_string()),
        }
    }
}
