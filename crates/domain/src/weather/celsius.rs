use crate::errors::DomainError;

const COLDEST_RECORDED: f64 = -90.0;
const HOTTEST_RECORDED: f64 = 60.0;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Celsius(f64);

impl Celsius {
    pub fn new(degrees: f64) -> Result<Self, DomainError> {
        if !degrees.is_finite() {
            return Err(DomainError::InvalidInput(
                "a temperature must be a finite number".into(),
            ));
        }

        if !(COLDEST_RECORDED..=HOTTEST_RECORDED).contains(&degrees) {
            return Err(DomainError::InvalidInput(format!(
                "a temperature on earth is between {COLDEST_RECORDED} and {HOTTEST_RECORDED} degrees, got {degrees}"
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
