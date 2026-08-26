use chrono::{DateTime, Utc};

use super::{JobId, JobKind, JobStatus, JobSubject};

pub struct JobData {
    pub id: JobId,
    pub kind: JobKind,
    pub subject: JobSubject,
    pub status: JobStatus,
    pub attempts: u32,
    pub last_error: Option<String>,
    pub enqueued_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Job {
    id: JobId,
    kind: JobKind,
    subject: JobSubject,
    status: JobStatus,
    attempts: u32,
    last_error: Option<String>,
    enqueued_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Job {
    pub fn pending(kind: JobKind, subject: JobSubject) -> Self {
        let now = Utc::now();

        Self {
            id: JobId::generate(),
            kind,
            subject,
            status: JobStatus::Pending,
            attempts: 0,
            last_error: None,
            enqueued_at: now,
            updated_at: now,
        }
    }

    pub fn from_persistence(data: JobData) -> Self {
        Self {
            id: data.id,
            kind: data.kind,
            subject: data.subject,
            status: data.status,
            attempts: data.attempts,
            last_error: data.last_error,
            enqueued_at: data.enqueued_at,
            updated_at: data.updated_at,
        }
    }
}

impl Job {
    pub fn id(&self) -> &JobId {
        &self.id
    }

    pub fn kind(&self) -> JobKind {
        self.kind
    }

    pub fn subject(&self) -> &JobSubject {
        &self.subject
    }

    pub fn status(&self) -> JobStatus {
        self.status
    }

    pub fn attempts(&self) -> u32 {
        self.attempts
    }

    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    pub fn enqueued_at(&self) -> &DateTime<Utc> {
        &self.enqueued_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    pub fn is_worth_another_attempt(&self, most_attempts: u32) -> bool {
        self.attempts < most_attempts
    }
}

impl Job {
    pub fn attempted(&mut self) {
        self.attempts += 1;
        self.updated_at = Utc::now();
    }
}
