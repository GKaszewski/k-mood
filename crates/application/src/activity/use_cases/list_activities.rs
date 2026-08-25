use std::sync::Arc;

use domain::activity::Activity;
use domain::ports::ActivityQueryPort;
use domain::user::UserId;

use crate::errors::ApplicationError;

pub struct Deps {
    pub query: Arc<dyn ActivityQueryPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn all(user_id: UserId, deps: &Deps) -> Result<Vec<Activity>, ApplicationError> {
    Ok(deps.query.find_by_user(&user_id).await?)
}

#[tracing::instrument(skip(deps))]
pub async fn active_only(user_id: UserId, deps: &Deps) -> Result<Vec<Activity>, ApplicationError> {
    Ok(deps.query.find_active_by_user(&user_id).await?)
}
