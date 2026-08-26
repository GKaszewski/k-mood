#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DailyMetricResponse {
    pub date: String,
    pub kind: String,
    pub value: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}
