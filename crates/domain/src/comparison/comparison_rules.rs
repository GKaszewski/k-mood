#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComparisonRules {
    pub minimum_sample_size: usize,
    pub false_discovery_rate: f64,
}
