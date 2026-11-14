#[derive(Debug, serde::Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct ComparedSpansParams {
    pub a_from: String,
    pub a_to: String,
    pub b_from: String,
    pub b_to: String,
}
