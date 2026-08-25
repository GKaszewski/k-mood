use crate::errors::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ContentType(String);

impl ContentType {
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into().trim().to_lowercase();
        if value.is_empty() {
            return Err(DomainError::InvalidInput(
                "content type cannot be empty".into(),
            ));
        }
        if !value.contains('/') {
            return Err(DomainError::InvalidInput(
                "content type must be in MIME format (e.g. image/jpeg)".into(),
            ));
        }
        Ok(Self(value))
    }

    pub fn from_persistence(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
