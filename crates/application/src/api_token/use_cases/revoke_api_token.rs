use std::sync::Arc;

use domain::api_token::ApiTokenId;
use domain::ports::ApiTokenCommandPort;
use domain::user::UserId;

use crate::errors::ApplicationError;

pub struct Deps {
    pub command: Arc<dyn ApiTokenCommandPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(user_id: UserId, id: ApiTokenId, deps: &Deps) -> Result<(), ApplicationError> {
    deps.command.revoke(&user_id, &id).await?;

    tracing::info!(%user_id, token_id = %id, "revoked an api token");

    Ok(())
}
