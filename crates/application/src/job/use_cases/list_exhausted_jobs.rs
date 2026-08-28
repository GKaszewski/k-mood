use std::sync::Arc;

use domain::job::Job;
use domain::ports::JobQueueQueryPort;

use crate::errors::ApplicationError;

pub struct Deps {
    pub jobs: Arc<dyn JobQueueQueryPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(most: usize, deps: &Deps) -> Result<Vec<Job>, ApplicationError> {
    Ok(deps.jobs.find_exhausted(most).await?)
}
