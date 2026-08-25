#[derive(Debug, Default, Clone, serde::Deserialize)]
#[serde(default)]
pub struct PushConfig {
    pub enabled: bool,
    pub vapid_private_key: Option<String>,
    pub vapid_subject: Option<String>,
}
