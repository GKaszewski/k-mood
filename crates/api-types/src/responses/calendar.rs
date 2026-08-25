use chrono::NaiveDate;

use super::EntryResponse;

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CalendarDayResponse {
    pub date: NaiveDate,
    pub entries: Vec<EntryResponse>,
    pub dominant_mood: Option<u8>,
    pub dominant_mood_label: Option<String>,
}
