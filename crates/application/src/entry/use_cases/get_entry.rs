use std::sync::Arc;

use domain::entry::{MoodEntry, MoodEntryId};
use domain::errors::DomainError;
use domain::ports::MoodEntryQueryPort;
use domain::user::UserId;

use crate::authorization::verify_ownership;
use crate::errors::ApplicationError;

pub struct Deps {
    pub query: Arc<dyn MoodEntryQueryPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    entry_id: MoodEntryId,
    caller_id: UserId,
    deps: &Deps,
) -> Result<MoodEntry, ApplicationError> {
    let entry = deps
        .query
        .find_by_id(&entry_id)
        .await?
        .ok_or_else(|| DomainError::NotFound("mood entry not found".into()))?;

    verify_ownership(entry.user_id(), &caller_id)?;
    Ok(entry)
}
