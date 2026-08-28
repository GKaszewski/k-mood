const DEFAULT_ENABLED: bool = true;
const DEFAULT_REQUESTS_PER_SECOND: u64 = 20;
const DEFAULT_BURST: u32 = 60;
const DEFAULT_TRUST_FORWARDED_FOR: bool = false;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(default)]
pub struct RateLimitConfig {
    pub enabled: bool,
    pub requests_per_second: u64,
    pub burst: u32,
    pub trust_forwarded_for: bool,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            enabled: DEFAULT_ENABLED,
            requests_per_second: DEFAULT_REQUESTS_PER_SECOND,
            burst: DEFAULT_BURST,
            trust_forwarded_for: DEFAULT_TRUST_FORWARDED_FOR,
        }
    }
}
