use super::Mood;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DayMood(f64);

impl DayMood {
    pub fn of(moods: &[Mood]) -> Option<Self> {
        if moods.is_empty() {
            return None;
        }

        let sum: u32 = moods.iter().map(|mood| u32::from(mood.value())).sum();

        Some(Self(f64::from(sum) / moods.len() as f64))
    }
}

impl DayMood {
    pub fn value(&self) -> f64 {
        self.0
    }

    pub fn rounded(&self) -> Mood {
        Mood::try_from(self.0.round() as u8).unwrap_or(Mood::Meh)
    }
}
