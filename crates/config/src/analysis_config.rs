const DEFAULT_MINIMUM_SAMPLE_SIZE: usize = 30;
const DEFAULT_FALSE_DISCOVERY_RATE: f64 = 0.10;
const DEFAULT_COMPARISON_MINIMUM_SAMPLE_SIZE: usize = 10;
const DEFAULT_MAXIMUM_COMPARISON_DAYS: i64 = 1830;
const DEFAULT_TOP_MUSIC_COUNT: usize = 10;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(default)]
pub struct AnalysisConfig {
    pub minimum_sample_size: usize,
    pub false_discovery_rate: f64,
    pub comparison_minimum_sample_size: usize,
    pub maximum_comparison_days: i64,
    pub top_music_count: usize,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            minimum_sample_size: DEFAULT_MINIMUM_SAMPLE_SIZE,
            false_discovery_rate: DEFAULT_FALSE_DISCOVERY_RATE,
            comparison_minimum_sample_size: DEFAULT_COMPARISON_MINIMUM_SAMPLE_SIZE,
            maximum_comparison_days: DEFAULT_MAXIMUM_COMPARISON_DAYS,
            top_music_count: DEFAULT_TOP_MUSIC_COUNT,
        }
    }
}
