#[derive(Debug, serde::Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct ListEntriesParams {
    pub from: Option<String>,
    pub to: Option<String>,
    pub mood: Option<u8>,
    pub activity: Option<String>,
    pub updated_since: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct DateRangeParams {
    pub from: String,
    pub to: String,
}
