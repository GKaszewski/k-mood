use std::sync::Arc;

use domain::errors::DomainError;
use domain::ports::{UserCommandPort, UserQueryPort};
use domain::user::User;

use crate::errors::ApplicationError;

use super::super::commands::UpdateProfileCommand;

pub struct Deps {
    pub user_command: Arc<dyn UserCommandPort>,
    pub user_query: Arc<dyn UserQueryPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(cmd: UpdateProfileCommand, deps: &Deps) -> Result<User, ApplicationError> {
    let mut user = deps
        .user_query
        .find_by_id(&cmd.user_id)
        .await?
        .ok_or_else(|| DomainError::NotFound("user not found".into()))?;

    user.update_display_name(cmd.display_name);
    user.update_timezone(cmd.timezone);

    deps.user_command.save(&user).await?;

    Ok(user)
}
