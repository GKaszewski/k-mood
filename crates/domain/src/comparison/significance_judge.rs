use crate::statistics::{FalseDiscoveryControl, PValue};

use super::{ComparisonRules, Measure, Significance};

enum Verdict {
    Decided(Significance),
    Tested(PValue),
}

pub(super) struct SignificanceJudge {
    rules: ComparisonRules,
    overlap: bool,
}

impl SignificanceJudge {
    pub(super) fn new(rules: ComparisonRules, overlap: bool) -> Self {
        Self { rules, overlap }
    }

    pub(super) fn judge(&self, measures: &[&Measure]) -> Vec<Significance> {
        let verdicts: Vec<Verdict> = measures
            .iter()
            .map(|measure| self.verdict(measure))
            .collect();
        let p_values: Vec<PValue> = verdicts
            .iter()
            .filter_map(|verdict| match verdict {
                Verdict::Tested(p_value) => Some(*p_value),
                Verdict::Decided(_) => None,
            })
            .collect();
        let mut held_up = FalseDiscoveryControl::at(self.rules.false_discovery_rate)
            .holds_up(&p_values)
            .into_iter();

        verdicts
            .into_iter()
            .map(|verdict| match verdict {
                Verdict::Decided(significance) => significance,
                Verdict::Tested(_) => match held_up.next() {
                    Some(true) => Significance::LikelyReal,
                    _ => Significance::Noise,
                },
            })
            .collect()
    }

    fn verdict(&self, measure: &Measure) -> Verdict {
        if matches!(measure, Measure::Figure { .. }) || self.overlap {
            return Verdict::Decided(Significance::NotTested);
        }
        let (sample_a, sample_b) = measure.samples();
        if sample_a.min(sample_b) < self.rules.minimum_sample_size {
            return Verdict::Decided(Significance::NotEnoughData);
        }
        let Some(p_value) = measure.p_value() else {
            return Verdict::Decided(Significance::NotTested);
        };
        if !measure.expected_counts_suffice() {
            return Verdict::Decided(Significance::NotEnoughData);
        }
        Verdict::Tested(p_value)
    }
}
