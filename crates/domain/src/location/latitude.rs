use crate::errors::DomainError;

const MIN_LATITUDE: f64 = -90.0;
const MAX_LATITUDE: f64 = 90.0;

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Latitude(f64);

impl Latitude {
    pub fn new(degrees: f64) -> Result<Self, DomainError> {
        if !degrees.is_finite() {
            return Err(DomainError::InvalidInput(
                "latitude must be a finite number".into(),
            ));
        }
        if !(MIN_LATITUDE..=MAX_LATITUDE).contains(&degrees) {
            return Err(DomainError::InvalidInput(format!(
                "latitude must be between {MIN_LATITUDE} and {MAX_LATITUDE}, got {degrees}"
            )));
        }
        Ok(Self(degrees))
    }

    pub fn from_persistence(degrees: f64) -> Self {
        Self(degrees)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}
