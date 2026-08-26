use std::sync::Arc;

use domain::ports::UserPreferencesQueryPort;
use domain::user::{UserId, UserPreferences};

use crate::errors::ApplicationError;

pub async fn preferences_of(
    user_id: &UserId,
    query: &Arc<dyn UserPreferencesQueryPort>,
) -> Result<UserPreferences, ApplicationError> {
    Ok(query
        .find_by_user(user_id)
        .await?
        .unwrap_or_else(|| UserPreferences::off_by_default(user_id.clone())))
}
