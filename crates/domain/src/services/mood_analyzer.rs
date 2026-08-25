use chrono::Utc;

use crate::activity::ActivityId;
use crate::entry::{Mood, MoodEntry};

pub struct MoodAnalyzerService;

impl MoodAnalyzerService {
    #[tracing::instrument(skip(entries), fields(entry_count = entries.len()))]
    pub fn average_mood(entries: &[MoodEntry]) -> Option<f64> {
        if entries.is_empty() {
            return None;
        }

        let sum: u32 = entries.iter().map(|e| e.mood().value() as u32).sum();
        Some(sum as f64 / entries.len() as f64)
    }

    #[tracing::instrument(skip(entries), fields(entry_count = entries.len()))]
    pub fn mood_frequency(entries: &[MoodEntry]) -> Vec<(Mood, usize)> {
        let mut counts = [0usize; 5];
        for entry in entries {
            let index = entry.mood().value() as usize - 1;
            counts[index] += 1;
        }

        let moods = [Mood::Awful, Mood::Bad, Mood::Meh, Mood::Good, Mood::Rad];
        moods
            .into_iter()
            .zip(counts)
            .filter(|(_, count)| *count > 0)
            .collect()
    }

    #[tracing::instrument(skip(entries), fields(entry_count = entries.len()))]
    pub fn current_streak(entries: &[MoodEntry]) -> usize {
        if entries.is_empty() {
            return 0;
        }

        let mut dates: Vec<_> = entries.iter().map(|e| e.logged_at().date_naive()).collect();
        dates.sort();
        dates.dedup();

        let today = Utc::now().date_naive();

        let last_date = *dates.last().unwrap();
        let diff_to_today = (today - last_date).num_days();
        if diff_to_today > 1 {
            return 0;
        }

        let mut streak = 1;
        for window in dates.windows(2).rev() {
            let diff = window[1] - window[0];
            if diff.num_days() == 1 {
                streak += 1;
            } else {
                break;
            }
        }

        streak
    }

    #[tracing::instrument(skip(entries), fields(entry_count = entries.len()))]
    pub fn activity_mood_correlation(
        entries: &[MoodEntry],
        activity_id: &ActivityId,
    ) -> Option<f64> {
        let with: Vec<_> = entries
            .iter()
            .filter(|e| e.activities().contains(activity_id))
            .collect();

        let without: Vec<_> = entries
            .iter()
            .filter(|e| !e.activities().contains(activity_id))
            .collect();

        let avg_with = Self::average_mood_from_refs(&with)?;
        let avg_without = Self::average_mood_from_refs(&without)?;

        Some(avg_with - avg_without)
    }

    fn average_mood_from_refs(entries: &[&MoodEntry]) -> Option<f64> {
        if entries.is_empty() {
            return None;
        }

        let sum: u32 = entries.iter().map(|e| e.mood().value() as u32).sum();
        Some(sum as f64 / entries.len() as f64)
    }
}
