use application::metric::commands::MetricChange;
use domain::entry::{Date, DateSpan};
use domain::metric::{DailyMetric, MetricKind, MetricValue};

use crate::errors::ApiValidationError;
use crate::requests::{DateSpanParams, MetricPayload};
use crate::responses::DailyMetricResponse;

impl From<DailyMetric> for DailyMetricResponse {
    fn from(metric: DailyMetric) -> Self {
        Self {
            date: metric.date().to_string(),
            kind: metric.kind().name().to_string(),
            value: metric.value().count(),
            provider: metric
                .source()
                .provider()
                .map(|name| name.value().to_string()),
        }
    }
}

impl MetricPayload {
    pub fn into_change(self) -> Result<MetricChange, ApiValidationError> {
        let kind = MetricKind::from_name(&self.kind).ok_or_else(|| {
            ApiValidationError::Invalid(format!("unknown metric kind: {}", self.kind))
        })?;

        match self.value {
            None => Ok(MetricChange::Cleared(kind)),
            Some(count) => Ok(MetricChange::Stated(MetricValue::of_kind(kind, count)?)),
        }
    }
}

impl DateSpanParams {
    pub fn into_span(self) -> Result<DateSpan, ApiValidationError> {
        Ok(DateSpan::new(
            parse_date(&self.from)?,
            parse_date(&self.to)?,
        )?)
    }
}

pub fn parse_date(date: &str) -> Result<Date, ApiValidationError> {
    date.parse()
        .map(Date::from_persistence)
        .map_err(|_| ApiValidationError::Invalid(format!("invalid date: {date}")))
}
