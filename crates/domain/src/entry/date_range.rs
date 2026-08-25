use chrono::{DateTime, FixedOffset};

use crate::errors::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateRange {
    start: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
}

impl DateRange {
    pub fn new(
        start: DateTime<FixedOffset>,
        end: DateTime<FixedOffset>,
    ) -> Result<Self, DomainError> {
        if start > end {
            return Err(DomainError::InvalidInput(
                "date range start must be before or equal to end".into(),
            ));
        }
        Ok(Self { start, end })
    }

    pub fn start(&self) -> &DateTime<FixedOffset> {
        &self.start
    }

    pub fn end(&self) -> &DateTime<FixedOffset> {
        &self.end
    }
}
