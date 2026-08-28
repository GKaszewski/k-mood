use std::collections::HashMap;
use std::sync::Arc;

use domain::activity::{Activity, ActivityId};
use domain::dimension::{DimensionValue, lookup};
use domain::errors::DomainError;
use domain::ports::ActivityQueryPort;
use domain::user::UserId;

use crate::errors::ApplicationError;

pub async fn verify_activities_may_be_tagged(
    owner: &UserId,
    wanted: &[DimensionValue],
    already_worn: &[ActivityId],
    activities: &Arc<dyn ActivityQueryPort>,
) -> Result<(), ApplicationError> {
    let tagged = lookup::activities_in(wanted);

    if tagged.is_empty() {
        return Ok(());
    }

    let catalogue = catalogue_of(owner, activities).await?;

    for activity_id in tagged {
        let activity = catalogue
            .get(activity_id)
            .ok_or_else(|| not_in_this_catalogue(activity_id))?;

        if activity.is_archived() && !already_worn.contains(activity_id) {
            return Err(retired(activity));
        }
    }

    Ok(())
}

async fn catalogue_of(
    owner: &UserId,
    activities: &Arc<dyn ActivityQueryPort>,
) -> Result<HashMap<ActivityId, Activity>, ApplicationError> {
    Ok(activities
        .find_by_user(owner)
        .await?
        .into_iter()
        .map(|activity| (activity.id().clone(), activity))
        .collect())
}

fn not_in_this_catalogue(activity_id: &ActivityId) -> ApplicationError {
    DomainError::Forbidden(format!(
        "activity {activity_id} is not in this account's catalogue"
    ))
    .into()
}

fn retired(activity: &Activity) -> ApplicationError {
    ApplicationError::Validation(format!(
        "{} is archived, so it cannot be tagged onto an entry it was not already on: unarchive it first",
        activity.name().value()
    ))
}
