use crate::entry::Date;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CycleStartRestore(Date);

impl CycleStartRestore {
    pub fn on(date: Date) -> Self {
        Self(date)
    }

    pub fn date(&self) -> Date {
        self.0
    }
}
