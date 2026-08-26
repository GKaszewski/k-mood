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

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct DateSpanParams {
    pub from: String,
    pub to: String,
}
