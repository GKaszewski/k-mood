use std::sync::Arc;

use domain::entry::Date;
use domain::ports::{CycleStartCommandPort, UserPreferencesQueryPort};
use domain::user::UserId;

use crate::errors::ApplicationError;
use crate::user::preferences::preferences_of;

pub struct Deps {
    pub command: Arc<dyn CycleStartCommandPort>,
    pub preferences: Arc<dyn UserPreferencesQueryPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(user_id: UserId, date: Date, deps: &Deps) -> Result<(), ApplicationError> {
    let preferences = preferences_of(&user_id, &deps.preferences).await?;

    if !preferences.tracks_cycle() {
        return Err(ApplicationError::Validation(
            "cycle tracking is off for this account: turn it on in settings first".into(),
        ));
    }

    deps.command.record(&user_id, &date).await?;

    Ok(())
}
