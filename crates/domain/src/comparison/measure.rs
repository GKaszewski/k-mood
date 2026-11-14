use crate::statistics::{
    PValue, two_proportion, welch_satterthwaite_degrees_of_freedom, welch_two_sample,
};

use super::finite::finite;

const MINIMUM_EXPECTED_CELL_COUNT: f64 = 5.0;

#[derive(Debug, Clone, PartialEq)]
pub enum Measure {
    Figure {
        a: Option<f64>,
        b: Option<f64>,
    },
    Mean {
        a: Vec<f64>,
        b: Vec<f64>,
    },
    Share {
        hits_a: usize,
        total_a: usize,
        hits_b: usize,
        total_b: usize,
    },
}

impl Measure {
    pub fn values(&self) -> (Option<f64>, Option<f64>) {
        match self {
            Self::Figure { a, b } => (a.and_then(finite), b.and_then(finite)),
            Self::Mean { a, b } => (mean(a), mean(b)),
            Self::Share {
                hits_a,
                total_a,
                hits_b,
                total_b,
            } => (fraction(*hits_a, *total_a), fraction(*hits_b, *total_b)),
        }
    }

    pub fn samples(&self) -> (usize, usize) {
        match self {
            Self::Figure { .. } => (0, 0),
            Self::Mean { a, b } => (a.len(), b.len()),
            Self::Share {
                total_a, total_b, ..
            } => (*total_a, *total_b),
        }
    }

    pub fn p_value(&self) -> Option<PValue> {
        match self {
            Self::Figure { .. } => None,
            Self::Mean { a, b } => {
                let statistic = finite(welch_two_sample(a, b)?)?;
                let degrees_of_freedom = finite(welch_satterthwaite_degrees_of_freedom(a, b)?)?;
                Some(PValue::from_t_statistic(statistic, degrees_of_freedom))
            }
            Self::Share {
                hits_a,
                total_a,
                hits_b,
                total_b,
            } => {
                let score = finite(two_proportion(*hits_a, *total_a, *hits_b, *total_b)?)?;
                Some(PValue::from_standard_score(score))
            }
        }
    }

    pub fn expected_counts_suffice(&self) -> bool {
        match self {
            Self::Figure { .. } | Self::Mean { .. } => true,
            Self::Share {
                hits_a,
                total_a,
                hits_b,
                total_b,
            } => expected_cells(*hits_a + *hits_b, *total_a, *total_b)
                .iter()
                .all(|expected| *expected >= MINIMUM_EXPECTED_CELL_COUNT),
        }
    }
}

fn expected_cells(hits: usize, total_a: usize, total_b: usize) -> [f64; 4] {
    let pooled = hits as f64 / (total_a + total_b) as f64;
    let (total_a, total_b) = (total_a as f64, total_b as f64);

    [
        total_a * pooled,
        total_a * (1.0 - pooled),
        total_b * pooled,
        total_b * (1.0 - pooled),
    ]
}

fn mean(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    finite(values.iter().sum::<f64>() / values.len() as f64)
}

fn fraction(hits: usize, total: usize) -> Option<f64> {
    if total == 0 {
        return None;
    }
    finite(hits as f64 / total as f64)
}
