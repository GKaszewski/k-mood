use std::sync::Arc;

use domain::api_token::ApiToken;
use domain::errors::DomainError;
use domain::ports::{ApiTokenCommandPort, ApiTokenQueryPort, ApiTokenSecretPort};

use crate::errors::ApplicationError;

pub struct Deps {
    pub query: Arc<dyn ApiTokenQueryPort>,
    pub command: Arc<dyn ApiTokenCommandPort>,
    pub secrets: Arc<dyn ApiTokenSecretPort>,
}

#[tracing::instrument(skip(secret, deps))]
pub async fn execute(secret: &str, deps: &Deps) -> Result<ApiToken, ApplicationError> {
    let digest = deps.secrets.digest(secret);

    let token = deps
        .query
        .find_by_digest(&digest)
        .await?
        .ok_or_else(|| DomainError::Unauthorized("this api token is not valid".into()))?;

    if let Err(error) = deps.command.mark_used(token.id()).await {
        tracing::warn!(token_id = %token.id(), %error, "could not record that a token was used");
    }

    Ok(token)
}
