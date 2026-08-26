use chrono::{DateTime, FixedOffset, NaiveDate};

use crate::user::Timezone;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date(NaiveDate);

impl Date {
    pub fn from_instant(instant: &DateTime<FixedOffset>, timezone: &Timezone) -> Self {
        Self(instant.with_timezone(&timezone.resolve()).date_naive())
    }

    pub fn from_persistence(date: NaiveDate) -> Self {
        Self(date)
    }
}

impl Date {
    pub fn value(&self) -> NaiveDate {
        self.0
    }

    pub fn previous(&self) -> Self {
        Self(self.0.pred_opt().unwrap_or(self.0))
    }

    pub fn next(&self) -> Self {
        Self(self.0.succ_opt().unwrap_or(self.0))
    }

    pub fn days_since(&self, other: &Date) -> i64 {
        (self.0 - other.0).num_days()
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
