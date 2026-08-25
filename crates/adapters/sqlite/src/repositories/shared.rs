use domain::errors::DomainError;

pub fn db_err(e: sqlx::Error) -> DomainError {
    DomainError::InvalidInput(format!("database error: {e}"))
}
