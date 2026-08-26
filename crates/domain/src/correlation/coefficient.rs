use crate::errors::DomainError;

const STRONGEST_NEGATIVE: f64 = -1.0;
const STRONGEST_POSITIVE: f64 = 1.0;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Coefficient(f64);

impl Coefficient {
    pub fn new(value: f64) -> Result<Self, DomainError> {
        if !value.is_finite() {
            return Err(DomainError::InvalidInput(
                "a correlation coefficient must be a finite number".into(),
            ));
        }

        if !(STRONGEST_NEGATIVE..=STRONGEST_POSITIVE).contains(&value) {
            return Err(DomainError::InvalidInput(format!(
                "a correlation coefficient must be between {STRONGEST_NEGATIVE} and {STRONGEST_POSITIVE}, got {value}"
            )));
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}
