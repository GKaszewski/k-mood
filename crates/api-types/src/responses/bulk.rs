use uuid::Uuid;

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BulkActionResponse {
    pub affected_count: u64,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ImportResultResponse {
    pub imported: u64,
    pub skipped: u64,
    pub errors: Vec<String>,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CorrelationResponse {
    pub activity_id: Uuid,
    pub correlation: Option<f64>,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MediaIdResponse {
    pub id: Uuid,
}
