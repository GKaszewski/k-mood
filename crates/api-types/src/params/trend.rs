#[derive(Debug, serde::Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct TrendParams {
    pub from: String,
    pub to: String,
    /// Lowercase English day name the calendar weeks start on; Monday when omitted.
    pub week_starts_on: Option<String>,
}
