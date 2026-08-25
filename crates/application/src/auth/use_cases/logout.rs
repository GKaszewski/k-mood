use std::sync::Arc;

use domain::ports::RefreshSessionCommandPort;

use crate::errors::ApplicationError;

pub struct Deps {
    pub refresh_session_command: Arc<dyn RefreshSessionCommandPort>,
}

#[tracing::instrument(skip_all)]
pub async fn execute(refresh_token: &str, deps: &Deps) -> Result<(), ApplicationError> {
    deps.refresh_session_command.revoke(refresh_token).await?;
    Ok(())
}
