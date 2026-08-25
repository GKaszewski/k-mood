use std::sync::Arc;

use domain::activity::ActivityId;
use domain::errors::DomainError;
use domain::events::{DomainEvent, EventEnvelope};
use domain::ports::{ActivityCommandPort, ActivityQueryPort, EventPublisherPort};
use domain::user::UserId;

use crate::authorization::verify_ownership;
use crate::errors::ApplicationError;

pub struct Deps {
    pub command: Arc<dyn ActivityCommandPort>,
    pub query: Arc<dyn ActivityQueryPort>,
    pub events: Arc<dyn EventPublisherPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn archive(
    activity_id: ActivityId,
    caller_id: UserId,
    deps: &Deps,
) -> Result<(), ApplicationError> {
    let mut activity = deps
        .query
        .find_by_id(&activity_id)
        .await?
        .ok_or_else(|| DomainError::NotFound("activity not found".into()))?;

    verify_ownership(activity.user_id(), &caller_id)?;

    activity.archive()?;
    deps.command.save(&activity).await?;

    let event = DomainEvent::ActivityArchived {
        activity_id: activity.id().clone(),
        user_id: activity.user_id().clone(),
    };
    deps.events.publish(EventEnvelope::wrap(event)).await?;

    Ok(())
}

#[tracing::instrument(skip(deps))]
pub async fn unarchive(
    activity_id: ActivityId,
    caller_id: UserId,
    deps: &Deps,
) -> Result<(), ApplicationError> {
    let mut activity = deps
        .query
        .find_by_id(&activity_id)
        .await?
        .ok_or_else(|| DomainError::NotFound("activity not found".into()))?;

    verify_ownership(activity.user_id(), &caller_id)?;

    activity.unarchive()?;
    deps.command.save(&activity).await?;

    let event = DomainEvent::ActivityUnarchived {
        activity_id: activity.id().clone(),
        user_id: activity.user_id().clone(),
    };
    deps.events.publish(EventEnvelope::wrap(event)).await?;

    Ok(())
}
