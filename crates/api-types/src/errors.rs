use domain::errors::DomainError;

#[derive(Debug, thiserror::Error)]
pub enum ApiValidationError {
    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error("{0}")]
    Invalid(String),
}
