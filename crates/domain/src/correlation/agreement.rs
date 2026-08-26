use super::Coefficient;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Agreement {
    agreeing: usize,
    applicable: usize,
}

impl Agreement {
    pub fn of(scores: &[Coefficient], applicable: usize) -> Self {
        let positive = scores.iter().filter(|score| score.value() > 0.0).count();
        let negative = scores.iter().filter(|score| score.value() < 0.0).count();

        Self {
            agreeing: positive.max(negative),
            applicable,
        }
    }
}

impl Agreement {
    pub fn agreeing(&self) -> usize {
        self.agreeing
    }

    pub fn applicable(&self) -> usize {
        self.applicable
    }
}
