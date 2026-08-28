#[derive(Debug, serde::Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct DateSpanParams {
    pub from: String,
    pub to: String,
}
