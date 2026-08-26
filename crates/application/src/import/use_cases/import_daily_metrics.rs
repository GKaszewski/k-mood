use std::sync::Arc;

use domain::entry::Date;
use domain::metric::{DailyMetric, MetricKind, MetricValue, Source};
use domain::ports::{DailyMetricCommandPort, RejectionCommandPort};
use domain::provider::ProviderName;
use domain::rejection::{RejectedMetric, RejectionDetail, RejectionOrigin};
use domain::user::UserId;

use crate::errors::ApplicationError;
use crate::import::commands::{ImportDailyMetricsCommand, ImportedDay, ImportedMetric};

#[derive(Debug)]
pub struct RejectionSummary {
    pub date: Option<String>,
    pub kind: String,
    pub reason: String,
}

#[derive(Debug)]
pub struct ImportOutcome {
    pub accepted: usize,
    pub superseded: usize,
    pub rejected: Vec<RejectionSummary>,
}

pub struct Deps {
    pub metrics: Arc<dyn DailyMetricCommandPort>,
    pub rejections: Arc<dyn RejectionCommandPort>,
}

#[tracing::instrument(skip(deps), fields(days = command.days.len()))]
pub async fn execute(
    command: ImportDailyMetricsCommand,
    deps: &Deps,
) -> Result<ImportOutcome, ApplicationError> {
    if command.days.len() > command.maximum_days {
        return Err(ApplicationError::Validation(format!(
            "an import may carry at most {} days, this one carries {}",
            command.maximum_days,
            command.days.len()
        )));
    }

    let mut writable = Vec::new();
    let mut refused = Vec::new();

    for day in &command.days {
        read_a_day(
            day,
            &command.user_id,
            &command.provider,
            &mut writable,
            &mut refused,
        );
    }

    let submitted = writable.len();
    let written = deps.metrics.save(&writable).await?;

    record(&refused, deps).await;

    tracing::info!(
        accepted = written,
        superseded = submitted - written,
        rejected = refused.len(),
        provider = command.provider.value(),
        "imported daily metrics"
    );

    Ok(ImportOutcome {
        accepted: written,
        superseded: submitted - written,
        rejected: refused.iter().map(summarise).collect(),
    })
}

fn read_a_day(
    day: &ImportedDay,
    user_id: &UserId,
    provider: &ProviderName,
    writable: &mut Vec<DailyMetric>,
    refused: &mut Vec<RejectedMetric>,
) {
    let Some(date) = day.date.parse().ok().map(Date::from_persistence) else {
        for metric in &day.metrics {
            refused.push(rejection(
                user_id,
                RejectionDetail::new(Some(provider.clone()), None, &metric.kind, metric.value),
                format!("{} is not a date", day.date),
            ));
        }

        return;
    };

    for metric in &day.metrics {
        match read_a_metric(metric) {
            Ok(value) => writable.push(DailyMetric::new(
                user_id.clone(),
                date,
                value,
                Source::Provider(provider.clone()),
            )),
            Err(reason) => refused.push(rejection(
                user_id,
                RejectionDetail::new(
                    Some(provider.clone()),
                    Some(date),
                    &metric.kind,
                    metric.value,
                ),
                reason,
            )),
        }
    }
}

fn read_a_metric(metric: &ImportedMetric) -> Result<MetricValue, String> {
    let kind = MetricKind::from_name(&metric.kind)
        .ok_or_else(|| format!("unknown metric kind: {}", metric.kind))?;

    let Some(count) = metric.value else {
        return Err("a provider cannot clear a reading: only the user can".into());
    };

    MetricValue::of_kind(kind, count).map_err(|error| error.to_string())
}

fn rejection(user_id: &UserId, detail: RejectionDetail, reason: String) -> RejectedMetric {
    RejectedMetric::new(user_id.clone(), RejectionOrigin::Import, detail, reason)
}

fn summarise(rejected: &RejectedMetric) -> RejectionSummary {
    RejectionSummary {
        date: rejected.detail().date().map(|date| date.to_string()),
        kind: rejected.detail().kind().to_string(),
        reason: rejected.reason().to_string(),
    }
}

async fn record(refused: &[RejectedMetric], deps: &Deps) {
    if refused.is_empty() {
        return;
    }

    if let Err(error) = deps.rejections.record(refused).await {
        tracing::warn!(%error, "could not write to the rejection trace");
    }
}
