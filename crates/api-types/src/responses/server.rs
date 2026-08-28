#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfoResponse {
    pub version: String,
    pub registration_open: bool,
    pub push_enabled: bool,
    pub weather_enabled: bool,
    pub now_playing_provider: Option<String>,
    pub access_token_ttl_seconds: u64,
    pub limits: ServerLimitsResponse,
    pub token_scopes: Vec<TokenScopeResponse>,
    pub metric_kinds: Vec<MetricKindResponse>,
    pub moods: Vec<MoodResponse>,
    pub weather_conditions: Vec<String>,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ServerLimitsResponse {
    pub max_body_size: usize,
    pub max_content_length: usize,
    pub max_photos: usize,
    pub max_voice_memos: usize,
    pub max_activities_per_entry: usize,
    pub max_entries_per_page: i64,
    pub max_import_days: usize,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TokenScopeResponse {
    pub name: String,
    pub describes: String,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MetricKindResponse {
    pub name: String,
    pub unit: String,
    pub minimum: i64,
    pub maximum: i64,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MoodResponse {
    pub value: u8,
    pub label: String,
}
