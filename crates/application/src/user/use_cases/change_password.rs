use std::sync::Arc;

use domain::errors::DomainError;
use domain::ports::{
    PasswordHasherPort, RefreshSessionCommandPort, UserCommandPort, UserQueryPort,
};

use crate::errors::ApplicationError;

use super::super::commands::ChangePasswordCommand;

pub struct Deps {
    pub user_command: Arc<dyn UserCommandPort>,
    pub user_query: Arc<dyn UserQueryPort>,
    pub password_hasher: Arc<dyn PasswordHasherPort>,
    pub refresh_sessions: Arc<dyn RefreshSessionCommandPort>,
}

#[tracing::instrument(skip_all, fields(user_id = %cmd.user_id))]
pub async fn execute(cmd: ChangePasswordCommand, deps: &Deps) -> Result<(), ApplicationError> {
    let mut user = deps
        .user_query
        .find_by_id(&cmd.user_id)
        .await?
        .ok_or_else(|| DomainError::NotFound("user not found".into()))?;

    let valid = deps
        .password_hasher
        .verify(&cmd.current_password, user.password_hash())?;

    if !valid {
        return Err(DomainError::Unauthorized("current password is incorrect".into()).into());
    }

    let new_hash = deps.password_hasher.hash(&cmd.new_password)?;
    user.update_password(new_hash);

    deps.user_command.save(&user).await?;
    deps.refresh_sessions.revoke_all_for_user(user.id()).await?;

    tracing::info!(user_id = %user.id(), "password changed and every session revoked");

    Ok(())
}
