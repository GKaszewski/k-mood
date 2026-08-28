use domain::job::{Job, JobSubject};

use crate::responses::ExhaustedJobResponse;

impl From<Job> for ExhaustedJobResponse {
    fn from(job: Job) -> Self {
        let JobSubject::Entry(entry_id) = job.subject();

        Self {
            id: job.id().value().to_string(),
            kind: job.kind().name().to_string(),
            subject: entry_id.value().to_string(),
            attempts: job.attempts(),
            last_error: job.last_error().map(|reason| reason.to_string()),
            enqueued_at: *job.enqueued_at(),
            updated_at: *job.updated_at(),
        }
    }
}
