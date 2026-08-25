use application::entry::use_cases::get_mood_stats::MoodStats;

use crate::responses::{MoodFrequency, MoodStatsResponse};

impl From<MoodStats> for MoodStatsResponse {
    fn from(stats: MoodStats) -> Self {
        Self {
            average: stats.average,
            frequency: stats
                .frequency
                .into_iter()
                .map(|(mood, count)| MoodFrequency {
                    mood: mood.value(),
                    mood_label: format!("{mood:?}"),
                    count,
                })
                .collect(),
            current_streak: stats.current_streak,
            total_entries: stats.total_entries,
        }
    }
}
