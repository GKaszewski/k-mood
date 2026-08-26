use std::sync::Arc;

use domain::job::JobKind;
use domain::ports::{JobQueueCommandPort, RecordingBackfillQueryPort};

use crate::errors::ApplicationError;

pub struct Deps {
    pub backlog: Arc<dyn RecordingBackfillQueryPort>,
    pub queue: Arc<dyn JobQueueCommandPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(most: usize, deps: &Deps) -> Result<usize, ApplicationError> {
    let stranded = deps.backlog.find_songs_without_a_recording(most).await?;
    let mut enqueued = 0;

    for song in &stranded {
        if deps
            .queue
            .enqueue(JobKind::BackfillRecordingIdentity, &song.subject())
            .await?
        {
            enqueued += 1;
        }
    }

    if enqueued > 0 {
        tracing::info!(
            enqueued,
            found = stranded.len(),
            "swept songs with no recording identity onto the queue"
        );
    }

    Ok(enqueued)
}
