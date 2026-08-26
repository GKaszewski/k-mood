use crate::errors::DomainError;

use super::normal::two_sided_tail;

const CERTAIN: f64 = 1.0;
const IMPOSSIBLE: f64 = 0.0;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PValue(f64);

impl PValue {
    pub fn new(value: f64) -> Result<Self, DomainError> {
        if !value.is_finite() || !(IMPOSSIBLE..=CERTAIN).contains(&value) {
            return Err(DomainError::InvalidInput(format!(
                "a p-value must be between {IMPOSSIBLE} and {CERTAIN}, got {value}"
            )));
        }

        Ok(Self(value))
    }

    pub fn from_standard_score(standard_score: f64) -> Self {
        Self(two_sided_tail(standard_score))
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}
