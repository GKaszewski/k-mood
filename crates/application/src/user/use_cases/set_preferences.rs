use std::sync::Arc;

use domain::ports::{UserPreferencesCommandPort, UserPreferencesQueryPort};
use domain::user::{UserId, UserPreferences};

use crate::errors::ApplicationError;
use crate::user::preferences::preferences_of;

pub struct Deps {
    pub command: Arc<dyn UserPreferencesCommandPort>,
    pub query: Arc<dyn UserPreferencesQueryPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    user_id: UserId,
    tracks_cycle: bool,
    deps: &Deps,
) -> Result<UserPreferences, ApplicationError> {
    let mut preferences = preferences_of(&user_id, &deps.query).await?;
    preferences.track_cycle(tracks_cycle);

    deps.command.save(&preferences).await?;

    Ok(preferences)
}
