use std::sync::Arc;

use domain::activity::Activity;
use domain::events::{DomainEvent, EventEnvelope};
use domain::ports::{ActivityCommandPort, EventPublisherPort};

use crate::errors::ApplicationError;

use super::super::commands::CreateActivityCommand;

pub struct Deps {
    pub activities: Arc<dyn ActivityCommandPort>,
    pub events: Arc<dyn EventPublisherPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    cmd: CreateActivityCommand,
    deps: &Deps,
) -> Result<Activity, ApplicationError> {
    let activity = Activity::new(cmd.user_id, cmd.name, cmd.category);

    deps.activities.save(&activity).await?;

    let event = DomainEvent::ActivityCreated {
        activity_id: activity.id().clone(),
        user_id: activity.user_id().clone(),
    };
    deps.events.publish(EventEnvelope::wrap(event)).await?;

    Ok(activity)
}
