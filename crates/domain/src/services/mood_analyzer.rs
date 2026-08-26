use crate::entry::{Date, Mood, MoodEntry};

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

    #[tracing::instrument(skip(dates), fields(date_count = dates.len()))]
    pub fn current_streak(dates: &[Date], today: Date) -> usize {
        let mut dates = dates.to_vec();
        dates.sort();
        dates.dedup();

        let Some(most_recent) = dates.last() else {
            return 0;
        };

        if today.days_since(most_recent) > 1 {
            return 0;
        }

        let mut streak = 1;
        for window in dates.windows(2).rev() {
            if window[1].days_since(&window[0]) != 1 {
                break;
            }
            streak += 1;
        }

        streak
    }
}
