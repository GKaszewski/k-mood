const LONGEST_DAILY: i64 = 62;
const LONGEST_WEEKLY: i64 = 366;
const LONGEST_DAILY_SINGLE: i64 = 31;
const LONGEST_WEEKLY_SINGLE: i64 = 366;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Granularity {
    Day,
    Week,
    Month,
}

impl Granularity {
    pub fn for_days(days: i64) -> Self {
        if days <= LONGEST_DAILY {
            Self::Day
        } else if days <= LONGEST_WEEKLY {
            Self::Week
        } else {
            Self::Month
        }
    }

    pub fn for_single_period(days: i64) -> Self {
        if days <= LONGEST_DAILY_SINGLE {
            Self::Day
        } else if days <= LONGEST_WEEKLY_SINGLE {
            Self::Week
        } else {
            Self::Month
        }
    }
}
