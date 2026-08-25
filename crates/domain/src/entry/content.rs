use crate::errors::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Content(String);

impl Content {
    pub fn new(text: impl Into<String>) -> Result<Self, DomainError> {
        let trimmed = text.into().trim().to_string();
        if trimmed.is_empty() {
            return Err(DomainError::InvalidInput("content cannot be empty".into()));
        }
        Ok(Self(trimmed))
    }

    pub fn from_persistence(text: String) -> Self {
        Self(text)
    }
}

impl Content {
    pub fn value(&self) -> &str {
        &self.0
    }
}
