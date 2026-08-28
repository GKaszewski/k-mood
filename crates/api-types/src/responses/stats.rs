#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MoodFrequencyResponse {
    pub mood: u8,
    pub mood_label: String,
    pub count: usize,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MoodStatsResponse {
    pub average: Option<f64>,
    pub frequency: Vec<MoodFrequencyResponse>,
    pub current_streak: usize,
    pub total_entries: usize,
    pub first_logged_on: Option<String>,
    pub last_logged_on: Option<String>,
}
