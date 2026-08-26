use chrono::{DateTime, Utc};

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProviderConnectionResponse {
    pub provider: String,
    pub connected_at: DateTime<Utc>,
}
