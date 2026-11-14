use std::collections::{BTreeMap, BTreeSet};

use crate::activity::ActivityId;
use crate::entry::{ClippedSpan, Date, DayMood, Mood};
use crate::metric::MetricKind;
use crate::weather::Condition;

use super::streaks::longest_consecutive_run;
use super::{Coverage, CycleQuarter, DayWeather, MusicSummary, PeriodDay, PeriodInput};

const GOOD_THRESHOLD: f64 = Mood::Good as u8 as f64;

#[derive(Debug, Clone, PartialEq)]
pub struct PeriodSummary {
    span: ClippedSpan,
    coverage: Coverage,
    days: Vec<PeriodDay>,
    metrics: BTreeMap<MetricKind, Vec<f64>>,
    tracks_cycle: bool,
    music: MusicSummary,
}

impl PeriodSummary {
    pub fn of(input: &PeriodInput) -> Self {
        let span = input.span.span();
        let mut days: Vec<PeriodDay> = input
            .days
            .iter()
            .filter(|day| span.contains(&day.date))
            .cloned()
            .collect();
        days.sort_by_key(|day| day.date);
        let coverage = Coverage {
            days_in_span: span.days().max(0) as usize,
            logged_days: days.len(),
            entries: days.iter().map(|day| day.entries).sum(),
        };
        let metrics = input
            .metrics
            .iter()
            .map(|(kind, by_date)| {
                let values = by_date
                    .iter()
                    .filter(|(date, _)| span.contains(date))
                    .map(|(_, value)| *value)
                    .collect();
                (*kind, values)
            })
            .collect();
        let music = MusicSummary::of(&days, input.top_music_count);

        Self {
            span: input.span,
            coverage,
            days,
            metrics,
            tracks_cycle: input.tracks_cycle,
            music,
        }
    }
}

impl PeriodSummary {
    pub fn span(&self) -> &ClippedSpan {
        &self.span
    }

    pub fn coverage(&self) -> Coverage {
        self.coverage
    }

    pub fn tracks_cycle(&self) -> bool {
        self.tracks_cycle
    }

    pub fn music(&self) -> &MusicSummary {
        &self.music
    }

    pub fn day_moods(&self) -> Vec<f64> {
        self.moods_where(|_| true)
    }

    pub fn mood_counts(&self) -> BTreeMap<Mood, usize> {
        let mut counts: BTreeMap<Mood, usize> = Mood::ALL.iter().map(|mood| (*mood, 0)).collect();
        for day in &self.days {
            *counts.entry(day.mood.rounded()).or_insert(0) += 1;
        }
        counts
    }

    pub fn longest_logging_streak(&self) -> usize {
        longest_consecutive_run(&self.dates_where(|_| true))
    }

    pub fn longest_good_streak(&self) -> usize {
        longest_consecutive_run(&self.dates_where(|day| is_good(day.mood)))
    }
}

impl PeriodSummary {
    pub fn activities(&self) -> BTreeSet<ActivityId> {
        self.days
            .iter()
            .flat_map(|day| day.activities.iter().cloned())
            .collect()
    }

    pub fn activity_days(&self, activity: &ActivityId) -> usize {
        self.days
            .iter()
            .filter(|day| day.activities.contains(activity))
            .count()
    }

    pub fn moods_with_activity(&self, activity: &ActivityId) -> Vec<f64> {
        self.moods_where(|day| day.activities.contains(activity))
    }

    pub fn metric_values(&self, kind: MetricKind) -> Vec<f64> {
        self.metrics.get(&kind).cloned().unwrap_or_default()
    }
}

impl PeriodSummary {
    pub fn days_with_weather(&self) -> usize {
        self.days
            .iter()
            .filter(|day| DayWeather::of(&day.weather).is_some())
            .count()
    }

    pub fn condition_days(&self, condition: Condition) -> usize {
        self.days
            .iter()
            .filter(|day| day_condition(day) == Some(condition))
            .count()
    }

    pub fn temperatures(&self) -> Vec<f64> {
        self.days
            .iter()
            .filter_map(|day| DayWeather::of(&day.weather))
            .map(|(_, temperature)| temperature)
            .collect()
    }

    pub fn moods_in(&self, condition: Condition) -> Vec<f64> {
        self.moods_where(|day| day_condition(day) == Some(condition))
    }

    pub fn moods_in_quarter(&self, quarter: CycleQuarter) -> Vec<f64> {
        self.moods_where(|day| {
            day.cycle
                .is_some_and(|position| CycleQuarter::of(position.progress()) == quarter)
        })
    }
}

impl PeriodSummary {
    fn moods_where(&self, keep: impl Fn(&PeriodDay) -> bool) -> Vec<f64> {
        self.days
            .iter()
            .filter(|day| keep(day))
            .map(|day| day.mood.value())
            .collect()
    }

    fn dates_where(&self, keep: impl Fn(&PeriodDay) -> bool) -> BTreeSet<Date> {
        self.days
            .iter()
            .filter(|day| keep(day))
            .map(|day| day.date)
            .collect()
    }
}

fn day_condition(day: &PeriodDay) -> Option<Condition> {
    DayWeather::of(&day.weather).map(|(condition, _)| condition)
}

fn is_good(mood: DayMood) -> bool {
    mood.value() >= GOOD_THRESHOLD
}
