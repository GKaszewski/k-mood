use std::sync::Arc;

use domain::ports::RejectionQueryPort;
use domain::rejection::RejectedMetric;
use domain::user::UserId;

use crate::errors::ApplicationError;

pub struct Deps {
    pub rejections: Arc<dyn RejectionQueryPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    user_id: UserId,
    deps: &Deps,
) -> Result<Vec<RejectedMetric>, ApplicationError> {
    Ok(deps.rejections.find_recent_by_user(&user_id).await?)
}
