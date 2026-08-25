#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MoodFrequency {
    pub mood: u8,
    pub mood_label: String,
    pub count: usize,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MoodStatsResponse {
    pub average: Option<f64>,
    pub frequency: Vec<MoodFrequency>,
    pub current_streak: usize,
    pub total_entries: usize,
}
