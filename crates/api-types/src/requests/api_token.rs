#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MintApiTokenRequest {
    pub name: String,
    pub scopes: Vec<String>,
}
