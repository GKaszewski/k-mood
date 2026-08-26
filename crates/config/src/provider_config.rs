#[derive(Debug, Default, Clone, serde::Deserialize)]
#[serde(default)]
pub struct ProviderConfig {
    pub encryption_key: Option<String>,
}
