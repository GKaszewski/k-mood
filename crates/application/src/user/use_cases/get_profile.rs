use std::sync::Arc;

use domain::errors::DomainError;
use domain::ports::UserQueryPort;
use domain::user::{User, UserId};

use crate::errors::ApplicationError;

pub struct Deps {
    pub user_query: Arc<dyn UserQueryPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(user_id: UserId, deps: &Deps) -> Result<User, ApplicationError> {
    deps.user_query
        .find_by_id(&user_id)
        .await?
        .ok_or_else(|| DomainError::NotFound("user not found".into()).into())
}
