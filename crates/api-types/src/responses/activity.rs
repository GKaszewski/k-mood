use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ActivityResponse {
    pub id: Uuid,
    pub name: String,
    pub category: Option<String>,
    pub archived: bool,
    pub created_at: DateTime<Utc>,
}
