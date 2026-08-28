use std::sync::Arc;

use domain::activity::ActivityId;
use domain::errors::DomainError;
use domain::ports::{ActivityCommandPort, ActivityQueryPort, MoodEntryQueryPort};
use domain::user::UserId;

use crate::authorization::verify_ownership;
use crate::errors::ApplicationError;

pub struct Deps {
    pub command: Arc<dyn ActivityCommandPort>,
    pub query: Arc<dyn ActivityQueryPort>,
    pub entries: Arc<dyn MoodEntryQueryPort>,
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
    refuse_while_entries_still_wear_it(&activity_id, deps).await?;

    deps.command.delete(&activity_id).await?;

    Ok(())
}

async fn refuse_while_entries_still_wear_it(
    activity_id: &ActivityId,
    deps: &Deps,
) -> Result<(), ApplicationError> {
    let tagged = deps.entries.count_tagged_with(activity_id).await?;

    if tagged > 0 {
        return Err(ApplicationError::Validation(format!(
            "{tagged} entries are tagged with this activity, so deleting it would rewrite them: archive it instead"
        )));
    }

    Ok(())
}
