use std::sync::Arc;

use domain::ports::ReminderQueryPort;
use domain::reminder::Reminder;
use domain::user::UserId;

use crate::errors::ApplicationError;

pub struct Deps {
    pub query: Arc<dyn ReminderQueryPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(user_id: UserId, deps: &Deps) -> Result<Vec<Reminder>, ApplicationError> {
    Ok(deps.query.find_by_user(&user_id).await?)
}
