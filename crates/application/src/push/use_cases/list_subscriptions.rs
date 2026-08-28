use std::sync::Arc;

use domain::ports::PushSubscriptionQueryPort;
use domain::push::PushSubscription;
use domain::user::UserId;

use crate::errors::ApplicationError;

pub struct Deps {
    pub push_query: Arc<dyn PushSubscriptionQueryPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    user_id: UserId,
    deps: &Deps,
) -> Result<Vec<PushSubscription>, ApplicationError> {
    Ok(deps.push_query.find_by_user(&user_id).await?)
}
