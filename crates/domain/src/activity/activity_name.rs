use crate::errors::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ActivityName(String);

impl ActivityName {
    pub fn new(name: impl Into<String>) -> Result<Self, DomainError> {
        let trimmed = name.into().trim().to_string();
        if trimmed.is_empty() {
            return Err(DomainError::InvalidInput(
                "activity name cannot be empty".into(),
            ));
        }
        Ok(Self(trimmed))
    }

    pub fn from_persistence(name: String) -> Self {
        Self(name)
    }
}

impl ActivityName {
    pub fn value(&self) -> &str {
        &self.0
    }
}
