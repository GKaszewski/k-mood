use sqlx::SqlitePool;

use domain::errors::DomainError;
use domain::job::{Job, JobId, JobKind, JobStatus, JobSubject};

use super::super::shared::db_err;
use super::rows::{JobRow, readable};

const COLUMNS: &str = "id, kind, subject, status, attempts, last_error, enqueued_at, updated_at";

pub struct SqliteJobQueueRepository {
    pool: SqlitePool,
}

impl SqliteJobQueueRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl domain::ports::JobQueueCommandPort for SqliteJobQueueRepository {
    async fn enqueue(&self, kind: JobKind, subject: &JobSubject) -> Result<bool, DomainError> {
        let job = Job::pending(kind, subject.clone());

        let written = sqlx::query(
            "INSERT INTO jobs (id, kind, subject, status, attempts, last_error, enqueued_at, updated_at)
             VALUES (?, ?, ?, ?, 0, NULL, ?, ?)
             ON CONFLICT(kind, subject) DO NOTHING",
        )
        .bind(job.id().value().to_string())
        .bind(kind.name())
        .bind(subject.key())
        .bind(JobStatus::Pending.name())
        .bind(job.enqueued_at().to_rfc3339())
        .bind(job.updated_at().to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(db_err)?;

        Ok(written.rows_affected() > 0)
    }

    async fn claim(&self, kind: JobKind, most: usize) -> Result<Vec<Job>, DomainError> {
        let sql = format!(
            "UPDATE jobs SET status = ?, updated_at = ?
             WHERE id IN (
                 SELECT id FROM jobs WHERE kind = ? AND status = ?
                 ORDER BY enqueued_at LIMIT ?
             )
             RETURNING {COLUMNS}"
        );

        let rows: Vec<JobRow> = sqlx::query_as(sqlx::AssertSqlSafe(sql))
            .bind(JobStatus::Running.name())
            .bind(chrono::Utc::now().to_rfc3339())
            .bind(kind.name())
            .bind(JobStatus::Pending.name())
            .bind(most as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(db_err)?;

        let mut claimed = Vec::with_capacity(rows.len());

        for row in &rows {
            match readable(row) {
                Some(job) => claimed.push(job),
                None => self.abandon_unreadable(row).await?,
            }
        }

        Ok(claimed)
    }

    async fn finish(&self, id: &JobId) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM jobs WHERE id = ?")
            .bind(id.value().to_string())
            .execute(&self.pool)
            .await
            .map_err(db_err)?;

        Ok(())
    }

    async fn release(&self, id: &JobId, reason: &str) -> Result<(), DomainError> {
        self.settle(id, JobStatus::Pending, reason).await
    }

    async fn exhaust(&self, id: &JobId, reason: &str) -> Result<(), DomainError> {
        self.settle(id, JobStatus::Exhausted, reason).await
    }

    async fn reclaim_stalled(&self, stalled_after_seconds: i64) -> Result<u64, DomainError> {
        let stalled_before =
            chrono::Utc::now() - chrono::Duration::seconds(stalled_after_seconds.max(0));

        let reclaimed = sqlx::query(
            "UPDATE jobs SET status = ?, updated_at = ?
             WHERE status = ? AND updated_at <= ?",
        )
        .bind(JobStatus::Pending.name())
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(JobStatus::Running.name())
        .bind(stalled_before.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(db_err)?;

        Ok(reclaimed.rows_affected())
    }
}

impl SqliteJobQueueRepository {
    async fn abandon_unreadable(&self, row: &JobRow) -> Result<(), DomainError> {
        sqlx::query("UPDATE jobs SET status = ?, last_error = ?, updated_at = ? WHERE id = ?")
            .bind(JobStatus::Exhausted.name())
            .bind("this job is stored in a shape this build cannot read")
            .bind(chrono::Utc::now().to_rfc3339())
            .bind(&row.id)
            .execute(&self.pool)
            .await
            .map_err(db_err)?;

        Ok(())
    }

    async fn settle(&self, id: &JobId, status: JobStatus, reason: &str) -> Result<(), DomainError> {
        sqlx::query(
            "UPDATE jobs SET status = ?, attempts = attempts + 1, last_error = ?, updated_at = ?
             WHERE id = ?",
        )
        .bind(status.name())
        .bind(reason)
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(id.value().to_string())
        .execute(&self.pool)
        .await
        .map_err(db_err)?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl domain::ports::JobQueueQueryPort for SqliteJobQueueRepository {
    async fn find_exhausted(&self, most: usize) -> Result<Vec<Job>, DomainError> {
        let sql =
            format!("SELECT {COLUMNS} FROM jobs WHERE status = ? ORDER BY updated_at DESC LIMIT ?");

        let rows: Vec<JobRow> = sqlx::query_as(sqlx::AssertSqlSafe(sql))
            .bind(JobStatus::Exhausted.name())
            .bind(most as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(db_err)?;

        Ok(rows.iter().filter_map(readable).collect())
    }
}
