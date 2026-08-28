use chrono::{DateTime, Utc};

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ExhaustedJobResponse {
    pub id: String,
    pub kind: String,
    pub subject: String,
    pub attempts: u32,
    pub last_error: Option<String>,
    pub enqueued_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
