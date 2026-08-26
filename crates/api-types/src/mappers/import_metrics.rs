use application::import::commands::{ImportedDay, ImportedMetric};
use application::import::use_cases::import_daily_metrics::{ImportOutcome, RejectionSummary};
use domain::rejection::RejectedMetric;

use crate::requests::{ImportDailyMetricsRequest, ImportedDayPayload, ImportedMetricPayload};
use crate::responses::{ImportOutcomeResponse, RejectedMetricResponse, RejectionResponse};

impl ImportDailyMetricsRequest {
    pub fn into_days(self) -> Vec<ImportedDay> {
        self.days.into_iter().map(Into::into).collect()
    }
}

impl From<ImportedDayPayload> for ImportedDay {
    fn from(payload: ImportedDayPayload) -> Self {
        Self {
            date: payload.date,
            metrics: payload.metrics.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<ImportedMetricPayload> for ImportedMetric {
    fn from(payload: ImportedMetricPayload) -> Self {
        Self {
            kind: payload.kind,
            value: payload.value,
        }
    }
}

impl From<ImportOutcome> for ImportOutcomeResponse {
    fn from(outcome: ImportOutcome) -> Self {
        Self {
            accepted: outcome.accepted,
            superseded: outcome.superseded,
            rejected: outcome.rejected.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<RejectionSummary> for RejectedMetricResponse {
    fn from(summary: RejectionSummary) -> Self {
        Self {
            date: summary.date,
            kind: summary.kind,
            reason: summary.reason,
        }
    }
}

impl From<RejectedMetric> for RejectionResponse {
    fn from(rejected: RejectedMetric) -> Self {
        Self {
            id: rejected.id().value().to_string(),
            origin: rejected.origin().name().to_string(),
            provider: rejected
                .detail()
                .provider()
                .map(|name| name.value().to_string()),
            date: rejected.detail().date().map(|date| date.to_string()),
            kind: rejected.detail().kind().to_string(),
            value: rejected.detail().value(),
            reason: rejected.reason().to_string(),
            recorded_at: *rejected.recorded_at(),
        }
    }
}
