use std::sync::Arc;

use domain::entry::DateSpan;
use domain::metric::DailyMetric;
use domain::ports::DailyMetricQueryPort;
use domain::user::UserId;

use crate::errors::ApplicationError;

pub struct Deps {
    pub metrics: Arc<dyn DailyMetricQueryPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    user_id: UserId,
    span: DateSpan,
    deps: &Deps,
) -> Result<Vec<DailyMetric>, ApplicationError> {
    Ok(deps.metrics.find_by_span(&user_id, &span).await?)
}
