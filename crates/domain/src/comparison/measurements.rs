use crate::entry::Mood;

use super::{Measure, PeriodSummary, Quantity};

pub(super) struct Measurements<'a> {
    pub(super) a: &'a PeriodSummary,
    pub(super) b: &'a PeriodSummary,
}

impl<'a> Measurements<'a> {
    pub(super) fn of(a: &'a PeriodSummary, b: &'a PeriodSummary) -> Self {
        Self { a, b }
    }

    pub(super) fn all(&self) -> Vec<(Quantity, Measure)> {
        let mut measured = self.descriptive();
        measured.extend(self.moods());
        measured.extend(self.activities());
        measured.extend(self.metrics());
        measured.extend(self.weather());
        measured.extend(self.cycle());
        measured
    }
}

impl Measurements<'_> {
    pub(super) fn figure(&self, read: impl Fn(&PeriodSummary) -> Option<f64>) -> Measure {
        Measure::Figure {
            a: read(self.a),
            b: read(self.b),
        }
    }

    pub(super) fn mean(&self, read: impl Fn(&PeriodSummary) -> Vec<f64>) -> Measure {
        Measure::Mean {
            a: read(self.a),
            b: read(self.b),
        }
    }

    pub(super) fn share(
        &self,
        hits: impl Fn(&PeriodSummary) -> usize,
        total: impl Fn(&PeriodSummary) -> usize,
    ) -> Measure {
        Measure::Share {
            hits_a: hits(self.a),
            total_a: total(self.a),
            hits_b: hits(self.b),
            total_b: total(self.b),
        }
    }
}

impl Measurements<'_> {
    fn descriptive(&self) -> Vec<(Quantity, Measure)> {
        vec![
            (
                Quantity::DaysInSpan,
                self.figure(|period| Some(period.coverage().days_in_span as f64)),
            ),
            (
                Quantity::LoggedDays,
                self.figure(|period| Some(period.coverage().logged_days as f64)),
            ),
            (
                Quantity::Coverage,
                self.figure(|period| period.coverage().ratio()),
            ),
            (
                Quantity::Entries,
                self.figure(|period| Some(period.coverage().entries as f64)),
            ),
            (
                Quantity::EntriesPerLoggedDay,
                self.figure(|period| period.coverage().entries_per_logged_day()),
            ),
            (Quantity::AverageMood, self.mean(PeriodSummary::day_moods)),
            (
                Quantity::LongestLoggingStreak,
                self.figure(|period| Some(period.longest_logging_streak() as f64)),
            ),
            (
                Quantity::LongestGoodStreak,
                self.figure(|period| Some(period.longest_good_streak() as f64)),
            ),
        ]
    }

    fn moods(&self) -> Vec<(Quantity, Measure)> {
        Mood::ALL
            .into_iter()
            .map(|mood| {
                let measure = self.share(
                    |period| period.mood_counts().get(&mood).copied().unwrap_or(0),
                    logged_days,
                );
                (Quantity::MoodShare(mood), measure)
            })
            .collect()
    }
}

pub(super) fn logged_days(period: &PeriodSummary) -> usize {
    period.coverage().logged_days
}
