#[derive(Debug, Clone, Copy, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum SignificanceResponse {
    LikelyReal,
    Noise,
    NotEnoughData,
    NotTested,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ComparisonRowResponse {
    pub key: String,
    pub a: Option<f64>,
    pub b: Option<f64>,
    pub absolute_change: Option<f64>,
    pub relative_change: Option<f64>,
    pub sample_a: usize,
    pub sample_b: usize,
    pub significance: SignificanceResponse,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MoodComparisonRowResponse {
    #[serde(flatten)]
    pub row: ComparisonRowResponse,
    pub mood: Option<u8>,
    pub mood_label: Option<String>,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ActivityComparisonRowResponse {
    #[serde(flatten)]
    pub row: ComparisonRowResponse,
    pub activity_id: String,
    pub name: String,
    pub category: Option<String>,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MetricComparisonRowResponse {
    #[serde(flatten)]
    pub row: ComparisonRowResponse,
    pub kind: String,
    pub unit: String,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct WeatherComparisonRowResponse {
    #[serde(flatten)]
    pub row: ComparisonRowResponse,
    pub condition: Option<String>,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CycleComparisonRowResponse {
    #[serde(flatten)]
    pub row: ComparisonRowResponse,
    pub quarter: u8,
}
