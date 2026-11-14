use crate::entry::Date;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TrendPoint {
    offset: usize,
    start: Date,
    days_covered: usize,
    mean_mood: Option<f64>,
    logged_days: usize,
    entries: usize,
}

impl TrendPoint {
    pub(super) fn new(
        offset: usize,
        start: Date,
        days_covered: usize,
        mean_mood: Option<f64>,
        logged_days: usize,
        entries: usize,
    ) -> Self {
        Self {
            offset,
            start,
            days_covered,
            mean_mood,
            logged_days,
            entries,
        }
    }
}

impl TrendPoint {
    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn start(&self) -> &Date {
        &self.start
    }

    pub fn days_covered(&self) -> usize {
        self.days_covered
    }

    pub fn mean_mood(&self) -> Option<f64> {
        self.mean_mood
    }

    pub fn logged_days(&self) -> usize {
        self.logged_days
    }

    pub fn entries(&self) -> usize {
        self.entries
    }
}
