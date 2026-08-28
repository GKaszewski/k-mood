use std::collections::HashSet;

use chrono::{DateTime, FixedOffset};

use domain::entry::{Mood, MoodEntry};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Fingerprint {
    instant: i64,
    mood: u8,
}

impl Fingerprint {
    fn of(logged_at: &DateTime<FixedOffset>, mood: Mood) -> Self {
        Self {
            instant: logged_at.timestamp(),
            mood: mood.value(),
        }
    }
}

pub struct AlreadyHere(HashSet<Fingerprint>);

impl AlreadyHere {
    pub fn holding(entries: &[MoodEntry]) -> Self {
        Self(
            entries
                .iter()
                .map(|entry| Fingerprint::of(entry.logged_at(), entry.mood()))
                .collect(),
        )
    }

    pub fn holds(&self, logged_at: &DateTime<FixedOffset>, mood: Mood) -> bool {
        self.0.contains(&Fingerprint::of(logged_at, mood))
    }

    pub fn remember(&mut self, logged_at: &DateTime<FixedOffset>, mood: Mood) {
        self.0.insert(Fingerprint::of(logged_at, mood));
    }
}
