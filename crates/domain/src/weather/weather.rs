use crate::provider::ProviderName;

use super::{Celsius, Condition};

#[derive(Debug, Clone, PartialEq)]
pub struct Weather {
    condition: Condition,
    temperature: Celsius,
    observed_by: ProviderName,
}

impl Weather {
    pub fn new(condition: Condition, temperature: Celsius, observed_by: ProviderName) -> Self {
        Self {
            condition,
            temperature,
            observed_by,
        }
    }
}

impl Weather {
    pub fn condition(&self) -> Condition {
        self.condition
    }

    pub fn temperature(&self) -> Celsius {
        self.temperature
    }

    pub fn observed_by(&self) -> &ProviderName {
        &self.observed_by
    }
}
