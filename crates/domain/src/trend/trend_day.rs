use crate::entry::{Date, DayMood};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TrendDay {
    date: Date,
    mood: DayMood,
    entries: usize,
}

impl TrendDay {
    pub fn new(date: Date, mood: DayMood, entries: usize) -> Self {
        Self {
            date,
            mood,
            entries,
        }
    }
}

impl TrendDay {
    pub fn date(&self) -> &Date {
        &self.date
    }

    pub fn mood(&self) -> &DayMood {
        &self.mood
    }

    pub fn entries(&self) -> usize {
        self.entries
    }
}
