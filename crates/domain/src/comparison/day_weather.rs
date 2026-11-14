use crate::weather::{Condition, Weather};

pub struct DayWeather;

impl DayWeather {
    pub fn of(readings: &[Weather]) -> Option<(Condition, f64)> {
        if readings.is_empty() {
            return None;
        }

        Some((
            Self::dominant_condition(readings),
            Self::mean_temperature(readings),
        ))
    }

    fn dominant_condition(readings: &[Weather]) -> Condition {
        let mut dominant = Condition::ALL[0];
        let mut dominant_count = 0;
        for condition in Condition::ALL {
            let count = readings
                .iter()
                .filter(|reading| reading.condition() == condition)
                .count();
            if count > dominant_count {
                dominant = condition;
                dominant_count = count;
            }
        }
        dominant
    }

    fn mean_temperature(readings: &[Weather]) -> f64 {
        let total: f64 = readings
            .iter()
            .map(|reading| reading.temperature().value())
            .sum();
        total / readings.len() as f64
    }
}
