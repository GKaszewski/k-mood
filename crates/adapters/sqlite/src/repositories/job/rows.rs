use domain::job::{Job, JobData, JobId, JobKind, JobStatus, JobSubject};

#[derive(sqlx::FromRow)]
pub struct JobRow {
    pub id: String,
    pub kind: String,
    pub subject: String,
    pub status: String,
    pub attempts: i64,
    pub last_error: Option<String>,
    pub enqueued_at: String,
    pub updated_at: String,
}

pub fn readable(row: &JobRow) -> Option<Job> {
    let job = row_to_job(row);

    if job.is_none() {
        tracing::warn!(
            job_id = %row.id,
            kind = %row.kind,
            status = %row.status,
            "skipped a stored job this build cannot read"
        );
    }

    job
}

fn row_to_job(row: &JobRow) -> Option<Job> {
    Some(Job::from_persistence(JobData {
        id: JobId::from_uuid(row.id.parse().ok()?),
        kind: JobKind::from_name(&row.kind)?,
        subject: JobSubject::from_key(&row.subject)?,
        status: JobStatus::from_name(&row.status)?,
        attempts: u32::try_from(row.attempts).ok()?,
        last_error: row.last_error.clone(),
        enqueued_at: row.enqueued_at.parse().ok()?,
        updated_at: row.updated_at.parse().ok()?,
    }))
}
