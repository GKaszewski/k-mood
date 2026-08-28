use std::sync::Arc;

use domain::attachment::MediaRef;
use domain::ports::MediaOwnershipPort;
use domain::user::UserId;

use crate::errors::ApplicationError;

pub struct Deps {
    pub ownership: Arc<dyn MediaOwnershipPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(owner: UserId, deps: &Deps) -> Result<Vec<MediaRef>, ApplicationError> {
    Ok(deps.ownership.owned_by(&owner).await?)
}
