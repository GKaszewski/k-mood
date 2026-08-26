use std::sync::Arc;

use chrono::{DateTime, Utc};

use domain::ports::ProviderConnectionQueryPort;
use domain::provider::ProviderName;
use domain::user::UserId;

use crate::errors::ApplicationError;

pub struct ConnectionSummary {
    pub provider: ProviderName,
    pub connected_at: DateTime<Utc>,
}

pub struct Deps {
    pub query: Arc<dyn ProviderConnectionQueryPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    user_id: UserId,
    deps: &Deps,
) -> Result<Vec<ConnectionSummary>, ApplicationError> {
    let connections = deps.query.find_by_user(&user_id).await?;

    Ok(connections
        .into_iter()
        .map(|connection| ConnectionSummary {
            provider: connection.provider().clone(),
            connected_at: *connection.created_at(),
        })
        .collect())
}
