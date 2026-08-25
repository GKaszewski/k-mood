use std::sync::Arc;

use domain::auth::{GeneratedToken, RefreshSession};
use domain::errors::DomainError;
use domain::ports::{AuthServicePort, RefreshSessionCommandPort, RefreshSessionQueryPort};

use crate::errors::ApplicationError;

pub struct RefreshResult {
    pub access_token: GeneratedToken,
    pub refresh_token: String,
}

pub struct Deps {
    pub auth_service: Arc<dyn AuthServicePort>,
    pub refresh_session_command: Arc<dyn RefreshSessionCommandPort>,
    pub refresh_session_query: Arc<dyn RefreshSessionQueryPort>,
    pub refresh_token_ttl_seconds: i64,
}

#[tracing::instrument(skip_all)]
pub async fn execute(
    old_refresh_token: &str,
    deps: &Deps,
) -> Result<RefreshResult, ApplicationError> {
    let session = deps
        .refresh_session_query
        .find_by_token(old_refresh_token)
        .await?
        .ok_or_else(|| DomainError::Unauthorized("invalid refresh token".into()))?;

    deps.refresh_session_command
        .revoke(old_refresh_token)
        .await?;

    if session.is_expired() {
        return Err(DomainError::Unauthorized("refresh token expired".into()).into());
    }

    let access_token = deps.auth_service.generate_token(session.user_id()).await?;

    let new_session =
        RefreshSession::new(session.user_id().clone(), deps.refresh_token_ttl_seconds);
    let refresh_token = new_session.token().to_string();
    deps.refresh_session_command.create(&new_session).await?;

    Ok(RefreshResult {
        access_token,
        refresh_token,
    })
}
