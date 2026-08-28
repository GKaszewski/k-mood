use super::EntryResponse;

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EntryPageResponse {
    pub items: Vec<EntryResponse>,
    pub total: u64,
    pub limit: i64,
    pub offset: i64,
    pub has_more: bool,
}
