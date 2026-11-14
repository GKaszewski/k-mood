use super::finite::finite;
use super::{Measure, Quantity, Significance};

#[derive(Debug, Clone, PartialEq)]
pub struct ComparisonRow {
    quantity: Quantity,
    a: Option<f64>,
    b: Option<f64>,
    absolute_change: Option<f64>,
    relative_change: Option<f64>,
    sample_a: usize,
    sample_b: usize,
    significance: Significance,
}

impl ComparisonRow {
    pub(super) fn new(quantity: Quantity, measure: &Measure, significance: Significance) -> Self {
        let (a, b) = measure.values();
        let (sample_a, sample_b) = measure.samples();
        Self {
            quantity,
            a,
            b,
            absolute_change: absolute_change(a, b),
            relative_change: relative_change(a, b),
            sample_a,
            sample_b,
            significance,
        }
    }
}

impl ComparisonRow {
    pub fn quantity(&self) -> &Quantity {
        &self.quantity
    }

    pub fn a(&self) -> Option<f64> {
        self.a
    }

    pub fn b(&self) -> Option<f64> {
        self.b
    }

    pub fn absolute_change(&self) -> Option<f64> {
        self.absolute_change
    }

    pub fn relative_change(&self) -> Option<f64> {
        self.relative_change
    }

    pub fn sample_a(&self) -> usize {
        self.sample_a
    }

    pub fn sample_b(&self) -> usize {
        self.sample_b
    }

    pub fn significance(&self) -> Significance {
        self.significance
    }
}

fn absolute_change(a: Option<f64>, b: Option<f64>) -> Option<f64> {
    finite(b? - a?)
}

fn relative_change(a: Option<f64>, b: Option<f64>) -> Option<f64> {
    let before = a?;
    if before == 0.0 {
        return None;
    }
    finite((b? - before) / before.abs())
}
