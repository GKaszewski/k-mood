use crate::errors::DomainError;

const MAX_LENGTH: usize = 100;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct DisplayName(String);

impl DisplayName {
    pub fn new(name: impl Into<String>) -> Result<Self, DomainError> {
        let trimmed = name.into().trim().to_string();
        if trimmed.is_empty() {
            return Err(DomainError::InvalidInput(
                "display name cannot be empty".into(),
            ));
        }
        if trimmed.len() > MAX_LENGTH {
            return Err(DomainError::InvalidInput(format!(
                "display name must be at most {MAX_LENGTH} characters"
            )));
        }
        Ok(Self(trimmed))
    }

    pub fn from_persistence(name: String) -> Self {
        Self(name)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
