use chrono::NaiveDate;

use crate::entry::Date;

const SYNODIC_MONTH_IN_DAYS: f64 = 29.530_588_853;
const NAMED_PHASES: [&str; 8] = [
    "New",
    "Waxing Crescent",
    "First Quarter",
    "Waxing Gibbous",
    "Full",
    "Waning Gibbous",
    "Last Quarter",
    "Waning Crescent",
];

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MoonPhase(f64);

impl MoonPhase {
    pub fn on(date: &Date) -> Self {
        let days = (date.value() - known_new_moon()).num_days() as f64;
        let through_the_cycle = days.rem_euclid(SYNODIC_MONTH_IN_DAYS) / SYNODIC_MONTH_IN_DAYS;

        Self(through_the_cycle)
    }
}

impl MoonPhase {
    pub fn illumination(&self) -> f64 {
        let lit = (1.0 - (std::f64::consts::TAU * self.0).cos()) / 2.0;

        lit.clamp(0.0, 1.0)
    }

    pub fn name(&self) -> &'static str {
        let segments = NAMED_PHASES.len() as f64;
        let segment = (self.0 * segments).round() as usize % NAMED_PHASES.len();

        NAMED_PHASES[segment]
    }
}

fn known_new_moon() -> NaiveDate {
    NaiveDate::from_ymd_opt(2000, 1, 6).unwrap_or_default()
}
