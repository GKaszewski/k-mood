use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DayScheduleResponse {
    pub monday: Option<String>,
    pub tuesday: Option<String>,
    pub wednesday: Option<String>,
    pub thursday: Option<String>,
    pub friday: Option<String>,
    pub saturday: Option<String>,
    pub sunday: Option<String>,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReminderResponse {
    pub id: Uuid,
    pub schedule: DayScheduleResponse,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
}
