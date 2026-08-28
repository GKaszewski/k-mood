use crate::dimension::DimensionPayload;

#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateEntryRequest {
    pub mood: u8,
    pub logged_at: Option<String>,
    pub dimensions: Option<Vec<DimensionPayload>>,
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateEntriesRequest {
    pub entries: Vec<CreateEntryRequest>,
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEntryRequest {
    pub mood: Option<u8>,
    pub logged_at: Option<String>,
    pub dimensions: Option<Vec<DimensionPayload>>,
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceActivityRequest {
    pub old_activity_id: String,
    pub new_activity_id: String,
}
