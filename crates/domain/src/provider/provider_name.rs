use crate::errors::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ProviderName(String);

impl ProviderName {
    pub fn new(name: impl Into<String>) -> Result<Self, DomainError> {
        let normalised = name.into().trim().to_lowercase();

        if normalised.is_empty() {
            return Err(DomainError::InvalidInput(
                "provider name cannot be empty".into(),
            ));
        }
        if !normalised
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-')
        {
            return Err(DomainError::InvalidInput(
                "provider name must contain only letters, digits and hyphens".into(),
            ));
        }

        Ok(Self(normalised))
    }

    pub fn from_persistence(name: String) -> Self {
        Self(name)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
