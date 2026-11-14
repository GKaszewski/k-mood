use super::PValue;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct FalseDiscoveryControl {
    false_discovery_rate: f64,
}

impl FalseDiscoveryControl {
    pub fn at(false_discovery_rate: f64) -> Self {
        Self {
            false_discovery_rate,
        }
    }

    pub fn holds_up(&self, p_values: &[PValue]) -> Vec<bool> {
        let tested = p_values.len();

        let mut by_size: Vec<usize> = (0..tested).collect();
        by_size.sort_by(|left, right| p_values[*left].value().total_cmp(&p_values[*right].value()));

        let largest_passing_rank = by_size
            .iter()
            .enumerate()
            .filter(|(position, index)| {
                p_values[**index].value() <= self.threshold_at(position + 1, tested)
            })
            .map(|(position, _)| position + 1)
            .max();

        let Some(rank) = largest_passing_rank else {
            return vec![false; tested];
        };

        let mut held = vec![false; tested];
        for index in &by_size[..rank] {
            held[*index] = true;
        }

        held
    }

    fn threshold_at(&self, rank: usize, tested: usize) -> f64 {
        rank as f64 / tested as f64 * self.false_discovery_rate
    }
}
