#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RestoreOutcomeResponse {
    pub entries: usize,
    pub metrics: usize,
    pub cycle_starts: usize,
    pub activities: usize,
    pub reminders: usize,
    pub media: usize,
    pub unreadable: Vec<String>,
}
