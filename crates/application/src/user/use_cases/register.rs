use std::sync::Arc;

use config::PresetConfig;
use domain::activity::{Activity, ActivityName, CategoryName};
use domain::errors::DomainError;
use domain::events::{DomainEvent, EventEnvelope};
use domain::ports::{
    ActivityCommandPort, EventPublisherPort, PasswordHasherPort, UserCommandPort, UserQueryPort,
};
use domain::user::{User, UserId};

use crate::errors::ApplicationError;

use super::super::commands::RegisterCommand;

pub struct Deps {
    pub user_command: Arc<dyn UserCommandPort>,
    pub user_query: Arc<dyn UserQueryPort>,
    pub activity_command: Arc<dyn ActivityCommandPort>,
    pub password_hasher: Arc<dyn PasswordHasherPort>,
    pub events: Arc<dyn EventPublisherPort>,
    pub preset: PresetConfig,
}

#[tracing::instrument(skip_all, fields(username = %cmd.username.value(), email = %cmd.email.value()))]
pub async fn execute(cmd: RegisterCommand, deps: &Deps) -> Result<User, ApplicationError> {
    if deps
        .user_query
        .find_by_username(&cmd.username)
        .await?
        .is_some()
    {
        return Err(DomainError::Conflict("username already taken".into()).into());
    }

    if deps.user_query.find_by_email(&cmd.email).await?.is_some() {
        return Err(DomainError::Conflict("email already registered".into()).into());
    }

    let password_hash = deps.password_hasher.hash(&cmd.password)?;
    let user = User::new(cmd.username, cmd.email, password_hash);

    deps.user_command.save(&user).await?;
    seed_default_activities(user.id(), deps).await?;

    let event = DomainEvent::UserRegistered {
        user_id: user.id().clone(),
    };
    deps.events.publish(EventEnvelope::wrap(event)).await?;

    Ok(user)
}

async fn seed_default_activities(user_id: &UserId, deps: &Deps) -> Result<(), ApplicationError> {
    for preset_activity in &deps.preset.activities {
        let name = match ActivityName::new(&preset_activity.name) {
            Ok(name) => name,
            Err(e) => {
                tracing::warn!(
                    activity_name = &preset_activity.name,
                    error = %e,
                    "skipped invalid preset activity name"
                );
                continue;
            }
        };

        let category = preset_activity.category.as_ref().and_then(|c| {
            CategoryName::new(c)
                .inspect_err(|e| {
                    tracing::warn!(category = c, error = %e, "skipped invalid preset category name");
                })
                .ok()
        });

        let activity = Activity::new(user_id.clone(), name, category);
        deps.activity_command.save(&activity).await?;
    }

    Ok(())
}
