const DEFAULT_ACCESS_TOKEN_TTL_SECONDS: u64 = 900;
const DEFAULT_REFRESH_TOKEN_TTL_SECONDS: u64 = 2_592_000;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(default)]
pub struct AuthConfig {
    pub jwt_secret: Option<String>,
    pub access_token_ttl_seconds: u64,
    pub refresh_token_ttl_seconds: u64,
    pub allow_registration: bool,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt_secret: None,
            access_token_ttl_seconds: DEFAULT_ACCESS_TOKEN_TTL_SECONDS,
            refresh_token_ttl_seconds: DEFAULT_REFRESH_TOKEN_TTL_SECONDS,
            allow_registration: true,
        }
    }
}
