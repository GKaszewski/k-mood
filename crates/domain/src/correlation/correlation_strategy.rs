use super::PValue;
use super::kendall::kendall_tau_b;
use super::mean_difference::mean_difference;
use super::pearson::pearson;
use super::ranks::average_ranks;
use super::welch::welch_standard_score;
use super::{Coefficient, CorrelationInput, Observation, SeriesShape};

const PEARSON: &str = "pearson";
const SPEARMAN: &str = "spearman";
const KENDALL: &str = "kendall";
const MEAN_DIFFERENCE: &str = "meanDifference";
const FEWEST_OBSERVATIONS: usize = 2;
const FEWEST_FOR_A_P_VALUE: usize = 4;
const SPEARMAN_VARIANCE_INFLATION: f64 = 1.06;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CorrelationStrategy {
    Pearson,
    Spearman,
    Kendall,
    MeanDifference,
}

impl CorrelationStrategy {
    pub const ALL: [CorrelationStrategy; 4] = [
        Self::Pearson,
        Self::Spearman,
        Self::Kendall,
        Self::MeanDifference,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Self::Pearson => PEARSON,
            Self::Spearman => SPEARMAN,
            Self::Kendall => KENDALL,
            Self::MeanDifference => MEAN_DIFFERENCE,
        }
    }

    pub fn scores(&self) -> SeriesShape {
        match self {
            Self::Pearson | Self::Spearman | Self::Kendall => SeriesShape::Continuous,
            Self::MeanDifference => SeriesShape::Presence,
        }
    }

    pub fn can_score(&self, input: &CorrelationInput) -> bool {
        self.scores() == input.series()
    }

    pub fn significance(&self, observations: &[Observation]) -> Option<PValue> {
        if observations.len() < FEWEST_FOR_A_P_VALUE {
            return None;
        }

        self.standard_score(observations)
            .map(PValue::from_standard_score)
    }

    fn standard_score(&self, observations: &[Observation]) -> Option<f64> {
        let (values, moods) = series(observations);
        let count = observations.len() as f64;

        match self {
            Self::Pearson => {
                let scored = pearson(&values, &moods)?;
                Some(fisher(scored, count, 1.0))
            }
            Self::Spearman => {
                let scored = pearson(&average_ranks(&values), &average_ranks(&moods))?;
                Some(fisher(scored, count, SPEARMAN_VARIANCE_INFLATION))
            }
            Self::Kendall => {
                let scored = kendall_tau_b(&values, &moods)?;
                Some(scored * (9.0 * count * (count - 1.0) / (2.0 * (2.0 * count + 5.0))).sqrt())
            }
            Self::MeanDifference => welch_standard_score(&values, &moods),
        }
    }

    pub fn score(&self, observations: &[Observation]) -> Option<Coefficient> {
        if observations.len() < FEWEST_OBSERVATIONS {
            return None;
        }

        let (values, moods) = series(observations);

        let scored = match self {
            Self::Pearson => pearson(&values, &moods),
            Self::Spearman => pearson(&average_ranks(&values), &average_ranks(&moods)),
            Self::Kendall => kendall_tau_b(&values, &moods),
            Self::MeanDifference => mean_difference(&values, &moods),
        }?;

        Coefficient::new(scored.clamp(-1.0, 1.0)).ok()
    }
}

fn series(observations: &[Observation]) -> (Vec<f64>, Vec<f64>) {
    let values = observations.iter().map(Observation::value).collect();
    let moods = observations
        .iter()
        .map(|observation| observation.day_mood().value())
        .collect();

    (values, moods)
}

fn fisher(scored: f64, count: f64, variance_inflation: f64) -> f64 {
    scored.atanh() * ((count - 3.0) / variance_inflation).sqrt()
}
