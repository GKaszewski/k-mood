use crate::errors::DomainError;

const FIRST_DAY: u16 = 1;
const LONGEST_PLAUSIBLE_CYCLE: u16 = 90;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CycleDay(u16);

impl CycleDay {
    pub fn new(day: u16) -> Result<Self, DomainError> {
        if !(FIRST_DAY..=LONGEST_PLAUSIBLE_CYCLE).contains(&day) {
            return Err(DomainError::InvalidInput(format!(
                "a cycle day must be between {FIRST_DAY} and {LONGEST_PLAUSIBLE_CYCLE}, got {day}"
            )));
        }

        Ok(Self(day))
    }

    pub fn value(&self) -> u16 {
        self.0
    }
}
