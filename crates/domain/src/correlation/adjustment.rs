use std::collections::BTreeMap;

use super::{CorrelationStrategy, Family};
use crate::statistics::{FalseDiscoveryControl, PValue};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tested {
    pub family: Family,
    pub strategy: CorrelationStrategy,
    pub p_value: PValue,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Adjustment {
    false_discovery_rate: f64,
}

impl Adjustment {
    pub fn controlling_false_discovery_at(false_discovery_rate: f64) -> Self {
        Self {
            false_discovery_rate,
        }
    }

    pub fn holds_up_across(&self, tested: &[Tested]) -> Vec<bool> {
        let mut groups: BTreeMap<(Family, CorrelationStrategy), Vec<usize>> = BTreeMap::new();
        for (index, entry) in tested.iter().enumerate() {
            groups
                .entry((entry.family, entry.strategy))
                .or_default()
                .push(index);
        }

        let mut held = vec![false; tested.len()];

        for places in groups.values() {
            let p_values: Vec<PValue> = places.iter().map(|index| tested[*index].p_value).collect();

            for (place, holds) in places.iter().zip(self.holds_up(&p_values)) {
                held[*place] = holds;
            }
        }

        held
    }

    pub fn holds_up(&self, p_values: &[PValue]) -> Vec<bool> {
        FalseDiscoveryControl::at(self.false_discovery_rate).holds_up(p_values)
    }
}
