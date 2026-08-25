use std::sync::Arc;

use domain::activity::ActivityId;
use domain::errors::DomainError;
use domain::ports::{ActivityCommandPort, ActivityQueryPort};
use domain::user::UserId;

use crate::authorization::verify_ownership;
use crate::errors::ApplicationError;

pub struct Deps {
    pub command: Arc<dyn ActivityCommandPort>,
    pub query: Arc<dyn ActivityQueryPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    activity_id: ActivityId,
    caller_id: UserId,
    deps: &Deps,
) -> Result<(), ApplicationError> {
    let activity = deps
        .query
        .find_by_id(&activity_id)
        .await?
        .ok_or_else(|| DomainError::NotFound("activity not found".into()))?;

    verify_ownership(activity.user_id(), &caller_id)?;

    deps.command.delete(&activity_id).await?;

    Ok(())
}
