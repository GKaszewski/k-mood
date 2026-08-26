use chrono::{DateTime, FixedOffset, Utc};
use uuid::Uuid;

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EntryResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub mood: u8,
    pub mood_label: String,
    pub logged_at: DateTime<FixedOffset>,
    pub dimensions: Vec<crate::dimension::DimensionPayload>,
    pub photo_urls: Vec<String>,
    pub voice_memo_urls: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
