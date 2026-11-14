const SECOND_QUARTER_STARTS_AT: f64 = 0.25;
const THIRD_QUARTER_STARTS_AT: f64 = 0.5;
const FOURTH_QUARTER_STARTS_AT: f64 = 0.75;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CycleQuarter {
    First,
    Second,
    Third,
    Fourth,
}

impl CycleQuarter {
    pub const ALL: [CycleQuarter; 4] = [Self::First, Self::Second, Self::Third, Self::Fourth];

    pub fn of(progress: f64) -> Self {
        if progress >= FOURTH_QUARTER_STARTS_AT {
            Self::Fourth
        } else if progress >= THIRD_QUARTER_STARTS_AT {
            Self::Third
        } else if progress >= SECOND_QUARTER_STARTS_AT {
            Self::Second
        } else {
            Self::First
        }
    }
}
