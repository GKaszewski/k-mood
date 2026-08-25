#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateActivityRequest {
    pub name: String,
    pub category: Option<String>,
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RenameActivityRequest {
    pub new_name: String,
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SetCategoryRequest {
    pub category: Option<String>,
}
