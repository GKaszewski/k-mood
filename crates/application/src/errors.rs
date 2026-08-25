use domain::errors::DomainError;

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error("validation failed: {0}")]
    Validation(String),
}
