use chrono::{DateTime, Utc};

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ImportOutcomeResponse {
    pub accepted: usize,
    pub superseded: usize,
    pub rejected: Vec<RejectedMetricResponse>,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RejectedMetricResponse {
    pub date: Option<String>,
    pub kind: String,
    pub reason: String,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RejectionResponse {
    pub id: String,
    pub origin: String,
    pub provider: Option<String>,
    pub date: Option<String>,
    pub kind: String,
    pub value: Option<i64>,
    pub reason: String,
    pub recorded_at: DateTime<Utc>,
}
