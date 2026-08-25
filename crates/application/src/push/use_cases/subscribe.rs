use std::sync::Arc;

use domain::ports::{PushSubscriptionCommandPort, PushSubscriptionQueryPort};
use domain::push::PushSubscription;

use crate::errors::ApplicationError;

use super::super::commands::SubscribePushCommand;

pub struct Deps {
    pub push_command: Arc<dyn PushSubscriptionCommandPort>,
    pub push_query: Arc<dyn PushSubscriptionQueryPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(cmd: SubscribePushCommand, deps: &Deps) -> Result<(), ApplicationError> {
    if let Some(_existing) = deps.push_query.find_by_endpoint(&cmd.endpoint).await? {
        tracing::debug!(
            endpoint = cmd.endpoint,
            "push subscription already exists, updating"
        );
    }

    let subscription = PushSubscription::new(cmd.user_id, cmd.endpoint, cmd.p256dh, cmd.auth);
    deps.push_command.save(&subscription).await?;

    tracing::info!(user_id = %subscription.user_id(), "push subscription saved");
    Ok(())
}
