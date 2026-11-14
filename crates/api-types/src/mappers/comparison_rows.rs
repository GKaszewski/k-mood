use std::collections::BTreeMap;

use domain::activity::{Activity, ActivityId};
use domain::comparison::{ComparisonRow, CycleQuarter, Quantity, Significance};

use crate::responses::{
    ActivityComparisonRowResponse, ComparisonRowResponse, CycleComparisonRowResponse,
    MetricComparisonRowResponse, MoodComparisonRowResponse, SignificanceResponse,
    WeatherComparisonRowResponse,
};

const UNKNOWN_ACTIVITY_NAME: &str = "Unknown activity";

#[derive(Debug, Default)]
pub(super) struct Sections {
    pub overview: Vec<ComparisonRowResponse>,
    pub mood: Vec<MoodComparisonRowResponse>,
    pub activities: Vec<ActivityComparisonRowResponse>,
    pub metrics: Vec<MetricComparisonRowResponse>,
    pub weather: Vec<WeatherComparisonRowResponse>,
    pub cycle: Vec<CycleComparisonRowResponse>,
}

impl Sections {
    pub(super) fn route(rows: &[ComparisonRow], labels: &BTreeMap<ActivityId, Activity>) -> Self {
        let mut sections = Self::default();
        for row in rows {
            sections.place(row, labels);
        }
        sections
    }

    fn place(&mut self, row: &ComparisonRow, labels: &BTreeMap<ActivityId, Activity>) {
        let base = base_response(row);
        match row.quantity() {
            Quantity::DaysInSpan
            | Quantity::LoggedDays
            | Quantity::Coverage
            | Quantity::Entries
            | Quantity::EntriesPerLoggedDay => self.overview.push(base),
            Quantity::AverageMood
            | Quantity::LongestLoggingStreak
            | Quantity::LongestGoodStreak => self.mood.push(mood_row(base, None)),
            Quantity::MoodShare(mood) => self.mood.push(mood_row(base, Some(*mood))),
            Quantity::ActivityShare(id) | Quantity::MoodWithActivity(id) => {
                self.activities.push(activity_row(base, id, labels))
            }
            Quantity::MetricMean(kind)
            | Quantity::MetricDaysWithData(kind)
            | Quantity::MetricTotal(kind) => self.metrics.push(MetricComparisonRowResponse {
                row: base,
                kind: kind.name().to_string(),
                unit: kind.unit().to_string(),
            }),
            Quantity::WeatherShare(condition) | Quantity::MoodInWeather(condition) => {
                self.weather.push(WeatherComparisonRowResponse {
                    row: base,
                    condition: Some(condition.name().to_string()),
                })
            }
            Quantity::MeanTemperature => self.weather.push(WeatherComparisonRowResponse {
                row: base,
                condition: None,
            }),
            Quantity::MoodInCycleQuarter(quarter) => self.cycle.push(CycleComparisonRowResponse {
                row: base,
                quarter: quarter_number(*quarter),
            }),
        }
    }
}

fn mood_row(
    row: ComparisonRowResponse,
    mood: Option<domain::entry::Mood>,
) -> MoodComparisonRowResponse {
    MoodComparisonRowResponse {
        row,
        mood: mood.map(|mood| mood.value()),
        mood_label: mood.map(|mood| mood.label().to_string()),
    }
}

fn activity_row(
    row: ComparisonRowResponse,
    id: &ActivityId,
    labels: &BTreeMap<ActivityId, Activity>,
) -> ActivityComparisonRowResponse {
    let activity = labels.get(id);
    ActivityComparisonRowResponse {
        row,
        activity_id: id.value().to_string(),
        name: activity
            .map(|activity| activity.name().value().to_string())
            .unwrap_or_else(|| UNKNOWN_ACTIVITY_NAME.to_string()),
        category: activity
            .and_then(Activity::category)
            .map(|category| category.value().to_string()),
    }
}

fn base_response(row: &ComparisonRow) -> ComparisonRowResponse {
    ComparisonRowResponse {
        key: key_of(row.quantity()).to_string(),
        a: row.a(),
        b: row.b(),
        absolute_change: row.absolute_change(),
        relative_change: row.relative_change(),
        sample_a: row.sample_a(),
        sample_b: row.sample_b(),
        significance: row.significance().into(),
    }
}

impl From<Significance> for SignificanceResponse {
    fn from(significance: Significance) -> Self {
        match significance {
            Significance::LikelyReal => Self::LikelyReal,
            Significance::Noise => Self::Noise,
            Significance::NotEnoughData => Self::NotEnoughData,
            Significance::NotTested => Self::NotTested,
        }
    }
}

fn key_of(quantity: &Quantity) -> &'static str {
    match quantity {
        Quantity::DaysInSpan => "daysInSpan",
        Quantity::LoggedDays => "loggedDays",
        Quantity::Coverage => "coverage",
        Quantity::Entries => "entries",
        Quantity::EntriesPerLoggedDay => "entriesPerLoggedDay",
        Quantity::AverageMood => "averageMood",
        Quantity::LongestLoggingStreak => "longestLoggingStreak",
        Quantity::LongestGoodStreak => "longestGoodStreak",
        Quantity::MoodShare(_) => "moodShare",
        Quantity::ActivityShare(_) => "activityShare",
        Quantity::MoodWithActivity(_) => "moodWithActivity",
        Quantity::MetricMean(_) => "metricMean",
        Quantity::MetricDaysWithData(_) => "metricDaysWithData",
        Quantity::MetricTotal(_) => "metricTotal",
        Quantity::WeatherShare(_) => "weatherShare",
        Quantity::MeanTemperature => "meanTemperature",
        Quantity::MoodInWeather(_) => "moodInWeather",
        Quantity::MoodInCycleQuarter(_) => "moodInCycleQuarter",
    }
}

fn quarter_number(quarter: CycleQuarter) -> u8 {
    match quarter {
        CycleQuarter::First => 1,
        CycleQuarter::Second => 2,
        CycleQuarter::Third => 3,
        CycleQuarter::Fourth => 4,
    }
}
