#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MediaIdResponse {
    pub id: uuid::Uuid,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OwnedMediaResponse {
    pub id: String,
    pub kind: String,
    pub url: String,
}
