use super::{
    ActivityComparisonRowResponse, ComparisonRowResponse, CycleComparisonRowResponse,
    MetricComparisonRowResponse, MoodComparisonRowResponse, MoodFrequencyResponse,
    MusicComparisonResponse, TrendPointResponse, WeatherComparisonRowResponse,
};

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ComparisonResponse {
    pub a: PeriodResponse,
    pub b: PeriodResponse,
    pub spans_overlap: bool,
    pub trend: ComparedTrendResponse,
    pub overview: Vec<ComparisonRowResponse>,
    pub mood: MoodSectionResponse,
    pub activities: Vec<ActivityComparisonRowResponse>,
    pub metrics: Vec<MetricComparisonRowResponse>,
    pub weather: Vec<WeatherComparisonRowResponse>,
    pub music: MusicComparisonResponse,
    pub cycle: Option<Vec<CycleComparisonRowResponse>>,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PeriodResponse {
    pub from: String,
    pub to: String,
    pub clipped: bool,
    pub days_in_span: usize,
    pub logged_days: usize,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MoodSectionResponse {
    pub rows: Vec<MoodComparisonRowResponse>,
    pub distribution: MoodDistributionResponse,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MoodDistributionResponse {
    pub a: Vec<MoodFrequencyResponse>,
    pub b: Vec<MoodFrequencyResponse>,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ComparedTrendResponse {
    pub granularity: String,
    pub a: Vec<TrendPointResponse>,
    pub b: Vec<TrendPointResponse>,
}
