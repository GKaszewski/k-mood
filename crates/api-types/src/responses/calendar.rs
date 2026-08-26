use chrono::NaiveDate;

use super::EntryResponse;

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CalendarDayResponse {
    pub date: NaiveDate,
    pub entries: Vec<EntryResponse>,
    pub day_mood: Option<f64>,
    pub mood: Option<u8>,
    pub mood_label: Option<String>,
    pub cycle_day: Option<u16>,
}
