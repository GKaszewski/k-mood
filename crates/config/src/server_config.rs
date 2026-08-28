use super::rate_limit_config::RateLimitConfig;

const DEFAULT_HOST: &str = "0.0.0.0";
const DEFAULT_PORT: u16 = 3000;
const DEFAULT_MAX_BODY_SIZE: usize = 10 * 1024 * 1024;
const DEFAULT_SPA_DIR: &str = "spa/dist";

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(default)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub max_body_size: usize,
    pub spa_dir: String,
    pub cors: CorsConfig,
    pub rate_limit: RateLimitConfig,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: DEFAULT_HOST.into(),
            port: DEFAULT_PORT,
            max_body_size: DEFAULT_MAX_BODY_SIZE,
            spa_dir: DEFAULT_SPA_DIR.into(),
            cors: CorsConfig::default(),
            rate_limit: RateLimitConfig::default(),
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(default)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allow_any_origin: bool,
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            allowed_origins: Vec::new(),
            allow_any_origin: true,
        }
    }
}
