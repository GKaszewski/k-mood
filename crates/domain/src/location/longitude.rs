use crate::errors::DomainError;

const MIN_LONGITUDE: f64 = -180.0;
const MAX_LONGITUDE: f64 = 180.0;

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Longitude(f64);

impl Longitude {
    pub fn new(degrees: f64) -> Result<Self, DomainError> {
        if !degrees.is_finite() {
            return Err(DomainError::InvalidInput(
                "longitude must be a finite number".into(),
            ));
        }
        if !(MIN_LONGITUDE..=MAX_LONGITUDE).contains(&degrees) {
            return Err(DomainError::InvalidInput(format!(
                "longitude must be between {MIN_LONGITUDE} and {MAX_LONGITUDE}, got {degrees}"
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
