use std::sync::Arc;

use domain::job::JobKind;
use domain::ports::{JobQueueCommandPort, WeatherBacklogQueryPort};

use crate::errors::ApplicationError;

pub struct Deps {
    pub backlog: Arc<dyn WeatherBacklogQueryPort>,
    pub queue: Arc<dyn JobQueueCommandPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(most: usize, deps: &Deps) -> Result<usize, ApplicationError> {
    let stranded = deps.backlog.find_places_without_weather(most).await?;
    let mut enqueued = 0;

    for place in &stranded {
        if deps
            .queue
            .enqueue(JobKind::ObserveWeather, &place.subject())
            .await?
        {
            enqueued += 1;
        }
    }

    if enqueued > 0 {
        tracing::info!(
            enqueued,
            found = stranded.len(),
            "swept places with no weather onto the queue"
        );
    }

    Ok(enqueued)
}
