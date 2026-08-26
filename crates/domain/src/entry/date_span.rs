use crate::errors::DomainError;

use super::Date;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateSpan {
    start: Date,
    end: Date,
}

impl DateSpan {
    pub fn new(start: Date, end: Date) -> Result<Self, DomainError> {
        if start > end {
            return Err(DomainError::InvalidInput(
                "date span start must be on or before its end".into(),
            ));
        }

        Ok(Self { start, end })
    }
}

impl DateSpan {
    pub fn start(&self) -> &Date {
        &self.start
    }

    pub fn end(&self) -> &Date {
        &self.end
    }

    pub fn contains(&self, date: &Date) -> bool {
        self.start <= *date && *date <= self.end
    }
}
