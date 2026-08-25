use super::{AuthConfig, EntryConfig, PresetConfig, PushConfig, ServerConfig, StorageConfig};

#[derive(Debug, Default, Clone, serde::Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub entry: EntryConfig,
    pub storage: StorageConfig,
    pub auth: AuthConfig,
    pub push: PushConfig,
    pub preset: PresetConfig,
}
