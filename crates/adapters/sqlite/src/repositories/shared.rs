use chrono::{DateTime, FixedOffset, SecondsFormat, Utc};

use domain::errors::DomainError;

pub fn db_err(e: sqlx::Error) -> DomainError {
    DomainError::InvalidInput(format!("database error: {e}"))
}

pub fn sortable_instant(instant: &DateTime<FixedOffset>) -> String {
    instant
        .with_timezone(&Utc)
        .to_rfc3339_opts(SecondsFormat::Secs, false)
}
