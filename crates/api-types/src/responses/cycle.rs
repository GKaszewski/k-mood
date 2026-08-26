#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CycleViewResponse {
    pub tracking: bool,
    pub starts: Vec<String>,
    pub today: Option<CyclePositionResponse>,
    pub usual_length: i64,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CyclePositionResponse {
    pub day: u16,
    pub progress: f64,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PreferencesResponse {
    pub tracks_cycle: bool,
}
