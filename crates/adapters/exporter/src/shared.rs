use domain::errors::DomainError;

pub fn zip_err(error: zip::result::ZipError) -> DomainError {
    DomainError::InvalidInput(format!("zip error: {error}"))
}

pub fn io_err(error: std::io::Error) -> DomainError {
    DomainError::InvalidInput(format!("io error: {error}"))
}

pub fn json_err(error: serde_json::Error) -> DomainError {
    DomainError::InvalidInput(format!("json error: {error}"))
}
