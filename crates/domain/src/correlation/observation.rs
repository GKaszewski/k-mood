use crate::entry::DayMood;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Observation {
    value: f64,
    day_mood: DayMood,
}

impl Observation {
    pub fn new(value: f64, day_mood: DayMood) -> Self {
        Self { value, day_mood }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn day_mood(&self) -> DayMood {
        self.day_mood
    }
}
