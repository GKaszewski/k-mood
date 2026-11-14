use chrono::Days;

use crate::entry::{Date, DateSpan};

use super::{TrendDay, TrendPoint};

pub(super) struct BucketWindow {
    start: Date,
    covered: DateSpan,
}

impl BucketWindow {
    pub(super) fn clipped_to(start: Date, end: Date, span: &DateSpan) -> Option<Self> {
        let covered_start = start.max(*span.start());
        let covered_end = end.min(*span.end());
        let covered = DateSpan::new(covered_start, covered_end).ok()?;
        Some(Self { start, covered })
    }

    pub(super) fn into_point(self, offset: usize, days: &[TrendDay]) -> TrendPoint {
        let logged: Vec<&TrendDay> = days
            .iter()
            .filter(|day| self.covered.contains(day.date()))
            .collect();
        let mood_total: f64 = logged.iter().map(|day| day.mood().value()).sum();
        let mean_mood = (!logged.is_empty()).then(|| mood_total / logged.len() as f64);
        let entries = logged.iter().map(|day| day.entries()).sum();

        TrendPoint::new(
            offset,
            self.start,
            self.covered.days() as usize,
            mean_mood,
            logged.len(),
            entries,
        )
    }
}

pub(super) fn days_after(date: Date, day_count: u64) -> Option<Date> {
    date.value()
        .checked_add_days(Days::new(day_count))
        .map(Date::from_persistence)
}
