#[derive(Debug, Default, serde::Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct ListActivitiesParams {
    pub include_archived: Option<bool>,
}
