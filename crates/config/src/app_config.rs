use super::{
    AnalysisConfig, AuthConfig, EntryConfig, ImportConfig, PresetConfig, ProviderConfig,
    PushConfig, ServerConfig, StorageConfig, WorkerConfig,
};

#[derive(Debug, Default, Clone, serde::Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub entry: EntryConfig,
    pub storage: StorageConfig,
    pub auth: AuthConfig,
    pub push: PushConfig,
    pub preset: PresetConfig,
    pub provider: ProviderConfig,
    pub analysis: AnalysisConfig,
    pub import: ImportConfig,
    pub worker: WorkerConfig,
}
