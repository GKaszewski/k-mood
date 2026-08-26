use std::sync::Arc;

use domain::api_token::ApiToken;
use domain::ports::ApiTokenQueryPort;
use domain::user::UserId;

use crate::errors::ApplicationError;

pub struct Deps {
    pub query: Arc<dyn ApiTokenQueryPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(user_id: UserId, deps: &Deps) -> Result<Vec<ApiToken>, ApplicationError> {
    Ok(deps.query.find_by_user(&user_id).await?)
}
