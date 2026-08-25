use std::sync::Arc;

use domain::auth::{GeneratedToken, RefreshSession};
use domain::errors::DomainError;
use domain::ports::{
    AuthServicePort, PasswordHasherPort, RefreshSessionCommandPort, UserQueryPort,
};
use domain::user::{Email, User, Username};

use crate::errors::ApplicationError;
use crate::user::commands::LoginCommand;

pub struct AuthResult {
    pub user: User,
    pub access_token: GeneratedToken,
    pub refresh_token: String,
}

pub struct Deps {
    pub user_query: Arc<dyn UserQueryPort>,
    pub password_hasher: Arc<dyn PasswordHasherPort>,
    pub auth_service: Arc<dyn AuthServicePort>,
    pub refresh_session_command: Arc<dyn RefreshSessionCommandPort>,
    pub refresh_token_ttl_seconds: i64,
}

#[tracing::instrument(skip_all, fields(identifier = %cmd.identifier))]
pub async fn execute(cmd: LoginCommand, deps: &Deps) -> Result<AuthResult, ApplicationError> {
    let user = find_user_by_identifier(&cmd.identifier, &deps.user_query).await?;

    let valid = deps
        .password_hasher
        .verify(&cmd.password, user.password_hash())?;

    if !valid {
        tracing::warn!(user_id = %user.id(), "failed login attempt");
        return Err(DomainError::Unauthorized("invalid credentials".into()).into());
    }

    tracing::info!(user_id = %user.id(), "login successful");

    let access_token = deps.auth_service.generate_token(user.id()).await?;

    let session = RefreshSession::new(user.id().clone(), deps.refresh_token_ttl_seconds);
    let refresh_token = session.token().to_string();
    deps.refresh_session_command.create(&session).await?;

    Ok(AuthResult {
        user,
        access_token,
        refresh_token,
    })
}

async fn find_user_by_identifier(
    identifier: &str,
    query: &Arc<dyn UserQueryPort>,
) -> Result<User, ApplicationError> {
    if let Ok(email) = Email::new(identifier)
        && let Some(user) = query.find_by_email(&email).await?
    {
        return Ok(user);
    }

    if let Ok(username) = Username::new(identifier)
        && let Some(user) = query.find_by_username(&username).await?
    {
        return Ok(user);
    }

    Err(DomainError::Unauthorized("invalid credentials".into()).into())
}
