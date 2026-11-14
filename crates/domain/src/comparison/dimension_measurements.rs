use crate::metric::MetricKind;
use crate::weather::Condition;

use super::measurements::{Measurements, logged_days};
use super::{CycleQuarter, Measure, PeriodSummary, Quantity};

impl Measurements<'_> {
    pub(super) fn activities(&self) -> Vec<(Quantity, Measure)> {
        let mut activities = self.a.activities();
        activities.extend(self.b.activities());
        let shares = activities.iter().map(|activity| {
            let measure = self.share(|period| period.activity_days(activity), logged_days);
            (Quantity::ActivityShare(activity.clone()), measure)
        });
        let moods = activities.iter().map(|activity| {
            let measure = self.mean(|period| period.moods_with_activity(activity));
            (Quantity::MoodWithActivity(activity.clone()), measure)
        });
        shares.chain(moods).collect()
    }

    pub(super) fn metrics(&self) -> Vec<(Quantity, Measure)> {
        let means = MetricKind::ALL.map(|kind| {
            let measure = self.mean(|period| period.metric_values(kind));
            (Quantity::MetricMean(kind), measure)
        });
        let days_with_data = MetricKind::ALL.map(|kind| {
            let measure = self.figure(|period| Some(period.metric_values(kind).len() as f64));
            (Quantity::MetricDaysWithData(kind), measure)
        });
        let totals = MetricKind::ALL.map(|kind| {
            let measure = self.figure(|period| total(&period.metric_values(kind)));
            (Quantity::MetricTotal(kind), measure)
        });
        means
            .into_iter()
            .chain(days_with_data)
            .chain(totals)
            .collect()
    }

    pub(super) fn weather(&self) -> Vec<(Quantity, Measure)> {
        let shares = Condition::ALL.map(|condition| {
            let measure = self.share(
                |period| period.condition_days(condition),
                PeriodSummary::days_with_weather,
            );
            (Quantity::WeatherShare(condition), measure)
        });
        let temperature = (
            Quantity::MeanTemperature,
            self.mean(PeriodSummary::temperatures),
        );
        let moods = Condition::ALL.map(|condition| {
            let measure = self.mean(|period| period.moods_in(condition));
            (Quantity::MoodInWeather(condition), measure)
        });
        shares
            .into_iter()
            .chain([temperature])
            .chain(moods)
            .collect()
    }

    pub(super) fn cycle(&self) -> Vec<(Quantity, Measure)> {
        if !(self.a.tracks_cycle() && self.b.tracks_cycle()) {
            return Vec::new();
        }
        CycleQuarter::ALL
            .into_iter()
            .map(|quarter| {
                let measure = self.mean(|period| period.moods_in_quarter(quarter));
                (Quantity::MoodInCycleQuarter(quarter), measure)
            })
            .collect()
    }
}

fn total(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    Some(values.iter().sum())
}
