use application::entry::use_cases::get_mood_trend::MoodTrend;
use domain::entry::DateSpan;
use domain::trend::{Granularity, TrendPoint, WeekStart};

use crate::errors::ApiValidationError;
use crate::params::TrendParams;
use crate::responses::{MoodTrendResponse, TrendPointResponse};

use super::metric::parse_date;

const DAY: &str = "day";
const WEEK: &str = "week";
const MONTH: &str = "month";

impl TrendParams {
    pub fn into_query_parts(self) -> Result<(DateSpan, WeekStart), ApiValidationError> {
        let span = DateSpan::new(parse_date(&self.from)?, parse_date(&self.to)?)?;
        let week_start = match self.week_starts_on {
            None => WeekStart::default(),
            Some(name) => WeekStart::from_name(&name).ok_or_else(|| {
                ApiValidationError::Invalid(format!("unknown weekStartsOn: {name}"))
            })?,
        };

        Ok((span, week_start))
    }
}

impl From<MoodTrend> for MoodTrendResponse {
    fn from(trend: MoodTrend) -> Self {
        Self {
            granularity: granularity_name(trend.granularity).to_string(),
            points: trend.points.iter().map(Into::into).collect(),
        }
    }
}

impl From<&TrendPoint> for TrendPointResponse {
    fn from(point: &TrendPoint) -> Self {
        Self {
            offset: point.offset(),
            start: point.start().to_string(),
            days_covered: point.days_covered(),
            mean_mood: point.mean_mood(),
            logged_days: point.logged_days(),
            entries: point.entries(),
        }
    }
}

pub(super) fn granularity_name(granularity: Granularity) -> &'static str {
    match granularity {
        Granularity::Day => DAY,
        Granularity::Week => WEEK,
        Granularity::Month => MONTH,
    }
}
