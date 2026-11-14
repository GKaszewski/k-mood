use crate::errors::DomainError;

use super::{ClippedSpan, Date};

const INCLUSIVE_END_DAY: i64 = 1;

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

    pub fn days(&self) -> i64 {
        self.end.days_since(&self.start) + INCLUSIVE_END_DAY
    }

    pub fn overlaps(&self, other: &DateSpan) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    pub fn clipped_to(&self, today: Date) -> Option<ClippedSpan> {
        if self.start > today {
            return None;
        }
        if self.end <= today {
            return Some(ClippedSpan::new(*self, false));
        }
        let clipped_span = Self {
            start: self.start,
            end: today,
        };
        Some(ClippedSpan::new(clipped_span, true))
    }
}
