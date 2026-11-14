#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaximumSpan(i64);

impl MaximumSpan {
    pub fn of_days(days: i64) -> Self {
        Self(days)
    }

    pub fn days(&self) -> i64 {
        self.0
    }
}
