use crate::errors::DomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Timezone(chrono_tz::Tz);

impl Timezone {
    pub fn new(name: impl AsRef<str>) -> Result<Self, DomainError> {
        let trimmed = name.as_ref().trim();

        if trimmed.is_empty() {
            return Err(DomainError::InvalidInput("timezone cannot be empty".into()));
        }

        trimmed.parse::<chrono_tz::Tz>().map(Self).map_err(|_| {
            DomainError::InvalidInput(format!("{trimmed} is not a timezone in the IANA database"))
        })
    }

    pub fn from_persistence(name: &str) -> Result<Self, DomainError> {
        Self::new(name)
    }

    pub fn value(&self) -> &str {
        self.0.name()
    }

    pub fn resolve(&self) -> chrono_tz::Tz {
        self.0
    }
}
