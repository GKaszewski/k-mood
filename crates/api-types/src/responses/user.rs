use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub timezone: Option<String>,
    pub role: String,
    pub created_at: DateTime<Utc>,
}
