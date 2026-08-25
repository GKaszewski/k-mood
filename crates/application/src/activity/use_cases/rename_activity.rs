use std::sync::Arc;

use domain::errors::DomainError;
use domain::events::{DomainEvent, EventEnvelope};
use domain::ports::{ActivityCommandPort, ActivityQueryPort, EventPublisherPort};
use domain::user::UserId;

use crate::authorization::verify_ownership;
use crate::errors::ApplicationError;

use super::super::commands::RenameActivityCommand;

pub struct Deps {
    pub command: Arc<dyn ActivityCommandPort>,
    pub query: Arc<dyn ActivityQueryPort>,
    pub events: Arc<dyn EventPublisherPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    cmd: RenameActivityCommand,
    caller_id: UserId,
    deps: &Deps,
) -> Result<(), ApplicationError> {
    let mut activity = deps
        .query
        .find_by_id(&cmd.activity_id)
        .await?
        .ok_or_else(|| DomainError::NotFound("activity not found".into()))?;

    verify_ownership(activity.user_id(), &caller_id)?;

    let old_name = activity.name().clone();
    activity.rename(cmd.new_name.clone());
    deps.command.save(&activity).await?;

    let event = DomainEvent::ActivityRenamed {
        activity_id: activity.id().clone(),
        user_id: activity.user_id().clone(),
        old_name,
        new_name: cmd.new_name,
    };
    deps.events.publish(EventEnvelope::wrap(event)).await?;

    Ok(())
}
