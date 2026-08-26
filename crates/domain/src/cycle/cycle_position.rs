use super::CycleDay;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CyclePosition {
    day: CycleDay,
    progress: f64,
}

impl CyclePosition {
    pub fn new(day: CycleDay, progress: f64) -> Self {
        Self {
            day,
            progress: progress.clamp(0.0, 1.0),
        }
    }

    pub fn day(&self) -> CycleDay {
        self.day
    }

    pub fn progress(&self) -> f64 {
        self.progress
    }
}
