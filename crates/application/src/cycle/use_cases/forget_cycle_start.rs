use std::sync::Arc;

use domain::entry::Date;
use domain::ports::CycleStartCommandPort;
use domain::user::UserId;

use crate::errors::ApplicationError;

pub struct Deps {
    pub command: Arc<dyn CycleStartCommandPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(user_id: UserId, date: Date, deps: &Deps) -> Result<(), ApplicationError> {
    deps.command.forget(&user_id, &date).await?;

    Ok(())
}
