#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TrendPointResponse {
    pub offset: usize,
    pub start: String,
    pub days_covered: usize,
    pub mean_mood: Option<f64>,
    pub logged_days: usize,
    pub entries: usize,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MoodTrendResponse {
    pub granularity: String,
    pub points: Vec<TrendPointResponse>,
}
