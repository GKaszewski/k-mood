use crate::entry::DateSpan;
use crate::errors::DomainError;

use super::MaximumSpan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComparedSpans {
    a: DateSpan,
    b: DateSpan,
}

impl ComparedSpans {
    pub fn new(a: DateSpan, b: DateSpan, maximum: MaximumSpan) -> Result<Self, DomainError> {
        for span in [&a, &b] {
            if span.days() > maximum.days() {
                return Err(DomainError::InvalidInput(format!(
                    "a compared span may cover at most {} days",
                    maximum.days()
                )));
            }
        }

        Ok(Self { a, b })
    }

    pub fn a(&self) -> &DateSpan {
        &self.a
    }

    pub fn b(&self) -> &DateSpan {
        &self.b
    }

    pub fn overlap(&self) -> bool {
        self.a.overlaps(&self.b)
    }
}
