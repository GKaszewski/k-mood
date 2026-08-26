use crate::entry::Date;

use super::{CycleDay, CyclePosition};

const ASSUMED_CYCLE_LENGTH: i64 = 28;

pub struct CycleCalendar {
    starts: Vec<Date>,
}

impl CycleCalendar {
    pub fn new(starts: Vec<Date>) -> Self {
        let mut starts = starts;
        starts.sort();
        starts.dedup();

        Self { starts }
    }
}

impl CycleCalendar {
    pub fn starts(&self) -> &[Date] {
        &self.starts
    }

    pub fn position_on(&self, date: &Date) -> Option<CyclePosition> {
        let start = self.start_of_the_cycle_containing(date)?;
        let elapsed = date.days_since(start);

        let day = CycleDay::new(u16::try_from(elapsed + 1).ok()?).ok()?;
        let length = self.length_of_the_cycle_from(start) as f64;

        Some(CyclePosition::new(day, elapsed as f64 / length))
    }

    pub fn usual_length(&self) -> i64 {
        let mut observed: Vec<i64> = self
            .starts
            .windows(2)
            .map(|pair| pair[1].days_since(&pair[0]))
            .collect();

        if observed.is_empty() {
            return ASSUMED_CYCLE_LENGTH;
        }

        observed.sort_unstable();

        observed[observed.len() / 2]
    }

    fn start_of_the_cycle_containing(&self, date: &Date) -> Option<&Date> {
        self.starts.iter().rev().find(|start| *start <= date)
    }

    fn length_of_the_cycle_from(&self, start: &Date) -> i64 {
        self.starts
            .iter()
            .find(|next| *next > start)
            .map(|next| next.days_since(start))
            .unwrap_or_else(|| self.usual_length())
    }
}
