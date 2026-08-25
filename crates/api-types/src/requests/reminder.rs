#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateReminderRequest {
    pub monday: Option<String>,
    pub tuesday: Option<String>,
    pub wednesday: Option<String>,
    pub thursday: Option<String>,
    pub friday: Option<String>,
    pub saturday: Option<String>,
    pub sunday: Option<String>,
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateReminderRequest {
    pub schedule: Option<CreateReminderRequest>,
    pub enabled: Option<bool>,
}
