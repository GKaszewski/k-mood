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
    let known = deps.push_query.find_by_endpoint(&cmd.endpoint).await?;

    let subscription = match known {
        Some(mut existing) => {
            if existing.user_id() != &cmd.user_id {
                tracing::info!(
                    endpoint = cmd.endpoint,
                    from = %existing.user_id(),
                    to = %cmd.user_id,
                    "a device already subscribed is moving to another account"
                );
            }

            existing.renew(cmd.user_id, cmd.p256dh, cmd.auth);
            existing
        }
        None => PushSubscription::new(cmd.user_id, cmd.endpoint, cmd.p256dh, cmd.auth),
    };

    deps.push_command.save(&subscription).await?;

    tracing::info!(user_id = %subscription.user_id(), "push subscription saved");

    Ok(())
}
