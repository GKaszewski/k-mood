#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateEntryRequest {
    pub mood: u8,
    pub logged_at: Option<String>,
    pub activity_ids: Option<Vec<String>>,
    pub content: Option<String>,
    pub photo_ids: Option<Vec<String>>,
    pub voice_memo_ids: Option<Vec<String>>,
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEntryRequest {
    pub mood: u8,
    pub logged_at: Option<String>,
    pub activity_ids: Option<Vec<String>>,
    pub content: Option<String>,
    pub photo_ids: Option<Vec<String>>,
    pub voice_memo_ids: Option<Vec<String>>,
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct ListEntriesParams {
    pub from: Option<String>,
    pub to: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct DateRangeParams {
    pub from: String,
    pub to: String,
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceActivityRequest {
    pub old_activity_id: String,
    pub new_activity_id: String,
}
