#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coverage {
    pub days_in_span: usize,
    pub logged_days: usize,
    pub entries: usize,
}

impl Coverage {
    pub fn ratio(&self) -> Option<f64> {
        if self.days_in_span == 0 {
            return None;
        }
        Some(self.logged_days as f64 / self.days_in_span as f64)
    }

    pub fn entries_per_logged_day(&self) -> Option<f64> {
        if self.logged_days == 0 {
            return None;
        }
        Some(self.entries as f64 / self.logged_days as f64)
    }
}
