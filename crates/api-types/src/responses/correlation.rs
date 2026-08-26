#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CorrelationRowResponse {
    pub input: CorrelationInputResponse,
    pub sample_size: usize,
    pub agreement: AgreementResponse,
    pub scores: Vec<StrategyScoreResponse>,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum CorrelationInputResponse {
    Metric { metric: String },
    MoonPhase,
    CycleProgress,
    Temperature,
    Activity { activity_id: String, name: String },
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AgreementResponse {
    pub agreeing: usize,
    pub applicable: usize,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StrategyScoreResponse {
    pub strategy: String,
    pub coefficient: f64,
    pub held_up: bool,
}
