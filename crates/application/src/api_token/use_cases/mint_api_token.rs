use std::sync::Arc;

use domain::api_token::{ApiToken, MintedApiToken};
use domain::ports::{ApiTokenCommandPort, ApiTokenSecretPort};

use crate::api_token::commands::MintApiTokenCommand;
use crate::errors::ApplicationError;

pub struct Deps {
    pub command: Arc<dyn ApiTokenCommandPort>,
    pub secrets: Arc<dyn ApiTokenSecretPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    command: MintApiTokenCommand,
    deps: &Deps,
) -> Result<MintedApiToken, ApplicationError> {
    let secret = deps.secrets.mint();
    let digest = deps.secrets.digest(&secret);

    let token = ApiToken::new(command.user_id, command.name, digest, command.scopes);
    deps.command.save(&token).await?;

    tracing::info!(
        token_id = %token.id(),
        name = token.name().value(),
        scopes = %token.scopes(),
        "minted an api token"
    );

    Ok(MintedApiToken::new(token, secret))
}
