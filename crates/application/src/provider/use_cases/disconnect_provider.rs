use std::sync::Arc;

use domain::ports::ProviderConnectionCommandPort;
use domain::provider::ProviderName;
use domain::user::UserId;

use crate::errors::ApplicationError;

pub struct Deps {
    pub command: Arc<dyn ProviderConnectionCommandPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    user_id: UserId,
    provider: ProviderName,
    deps: &Deps,
) -> Result<(), ApplicationError> {
    deps.command.delete(&user_id, &provider).await?;
    Ok(())
}
