use std::sync::Arc;

use domain::metric::{DailyMetric, MetricKind};
use domain::ports::{DailyMetricCommandPort, UserQueryPort};

use crate::day::{timezone_for, today_in};
use crate::errors::ApplicationError;
use crate::metric::commands::{MetricChange, SetDailyMetricsCommand};

pub struct Deps {
    pub metrics: Arc<dyn DailyMetricCommandPort>,
    pub users: Arc<dyn UserQueryPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(command: SetDailyMetricsCommand, deps: &Deps) -> Result<(), ApplicationError> {
    let timezone = timezone_for(&command.user_id, &deps.users).await?;

    if command.date > today_in(&timezone) {
        return Err(ApplicationError::Validation(format!(
            "cannot record a metric for {}, which has not happened yet",
            command.date
        )));
    }

    reject_a_kind_named_twice(&command.changes)?;
    reject_clearing_by_anyone_but_the_user(&command)?;

    let cleared: Vec<MetricKind> = command
        .changes
        .iter()
        .filter_map(|change| match change {
            MetricChange::Cleared(kind) => Some(*kind),
            MetricChange::Stated(_) => None,
        })
        .collect();

    let stated: Vec<DailyMetric> = command
        .changes
        .iter()
        .filter_map(|change| match change {
            MetricChange::Stated(value) => Some(DailyMetric::new(
                command.user_id.clone(),
                command.date,
                *value,
                command.source.clone(),
            )),
            MetricChange::Cleared(_) => None,
        })
        .collect();

    if !cleared.is_empty() {
        deps.metrics
            .delete(&command.user_id, &command.date, &cleared)
            .await?;
    }

    if !stated.is_empty() {
        deps.metrics.save(&stated).await?;
    }

    Ok(())
}

fn reject_clearing_by_anyone_but_the_user(
    command: &SetDailyMetricsCommand,
) -> Result<(), ApplicationError> {
    if command.source.is_manual() {
        return Ok(());
    }

    let clears = command
        .changes
        .iter()
        .any(|change| matches!(change, MetricChange::Cleared(_)));

    if clears {
        return Err(ApplicationError::Validation(
            "a provider cannot clear a reading: only the user can".into(),
        ));
    }

    Ok(())
}

fn reject_a_kind_named_twice(changes: &[MetricChange]) -> Result<(), ApplicationError> {
    for (position, change) in changes.iter().enumerate() {
        let named_again = changes[position + 1..]
            .iter()
            .any(|later| later.kind() == change.kind());

        if named_again {
            return Err(ApplicationError::Validation(format!(
                "{} appears more than once in the same request",
                change.kind().name()
            )));
        }
    }

    Ok(())
}
