use chrono::{DateTime, Utc};

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ApiTokenResponse {
    pub id: String,
    pub name: String,
    pub scope: String,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MintedApiTokenResponse {
    pub token: ApiTokenResponse,
    pub secret: String,
}
