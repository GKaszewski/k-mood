const DEFAULT_MAXIMUM_DAYS_PER_IMPORT: usize = 90;
const DEFAULT_REJECTIONS_KEPT: usize = 200;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(default)]
pub struct ImportConfig {
    pub maximum_days_per_import: usize,
    pub rejections_kept: usize,
}

impl Default for ImportConfig {
    fn default() -> Self {
        Self {
            maximum_days_per_import: DEFAULT_MAXIMUM_DAYS_PER_IMPORT,
            rejections_kept: DEFAULT_REJECTIONS_KEPT,
        }
    }
}
