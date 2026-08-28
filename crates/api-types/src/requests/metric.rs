#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SetDailyMetricsRequest {
    pub metrics: Vec<MetricPayload>,
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MetricPayload {
    pub kind: String,
    pub value: Option<i64>,
}
