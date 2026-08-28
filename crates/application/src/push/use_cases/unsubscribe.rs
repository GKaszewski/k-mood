use std::sync::Arc;

use domain::ports::PushSubscriptionCommandPort;

use crate::errors::ApplicationError;

use super::super::commands::UnsubscribePushCommand;

pub struct Deps {
    pub push_command: Arc<dyn PushSubscriptionCommandPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(cmd: UnsubscribePushCommand, deps: &Deps) -> Result<(), ApplicationError> {
    deps.push_command
        .delete_by_endpoint(&cmd.user_id, &cmd.endpoint)
        .await?;

    tracing::info!(user_id = %cmd.user_id, endpoint = cmd.endpoint, "push subscription removed");

    Ok(())
}
