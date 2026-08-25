use std::sync::Arc;

use domain::activity::ActivityId;
use domain::errors::DomainError;
use domain::ports::{ActivityQueryPort, MoodEntryCommandPort};
use domain::user::UserId;

use crate::authorization::verify_ownership;
use crate::errors::ApplicationError;

pub struct Deps {
    pub entry_command: Arc<dyn MoodEntryCommandPort>,
    pub activity_query: Arc<dyn ActivityQueryPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    user_id: UserId,
    old_activity_id: ActivityId,
    new_activity_id: ActivityId,
    deps: &Deps,
) -> Result<u64, ApplicationError> {
    let new_activity = deps
        .activity_query
        .find_by_id(&new_activity_id)
        .await?
        .ok_or_else(|| DomainError::NotFound("target activity not found".into()))?;

    verify_ownership(new_activity.user_id(), &user_id)?;

    let updated = deps
        .entry_command
        .replace_activity(&user_id, &old_activity_id, &new_activity_id)
        .await?;

    Ok(updated)
}
