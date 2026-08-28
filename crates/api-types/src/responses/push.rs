use chrono::{DateTime, Utc};

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PushSubscriptionResponse {
    pub id: String,
    pub endpoint: String,
    pub created_at: DateTime<Utc>,
}
