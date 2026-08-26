#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ImportDailyMetricsRequest {
    pub days: Vec<ImportedDayPayload>,
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ImportedDayPayload {
    pub date: String,
    pub metrics: Vec<ImportedMetricPayload>,
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ImportedMetricPayload {
    pub kind: String,
    pub value: Option<i64>,
}
