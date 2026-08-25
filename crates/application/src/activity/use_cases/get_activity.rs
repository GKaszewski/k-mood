use std::sync::Arc;

use domain::activity::{Activity, ActivityId};
use domain::errors::DomainError;
use domain::ports::ActivityQueryPort;
use domain::user::UserId;

use crate::authorization::verify_ownership;
use crate::errors::ApplicationError;

pub struct Deps {
    pub query: Arc<dyn ActivityQueryPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    activity_id: ActivityId,
    caller_id: UserId,
    deps: &Deps,
) -> Result<Activity, ApplicationError> {
    let activity = deps
        .query
        .find_by_id(&activity_id)
        .await?
        .ok_or_else(|| DomainError::NotFound("activity not found".into()))?;

    verify_ownership(activity.user_id(), &caller_id)?;
    Ok(activity)
}
