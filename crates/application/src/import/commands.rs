use domain::provider::ProviderName;
use domain::user::UserId;

#[derive(Debug)]
pub struct ImportCommand {
    pub user_id: UserId,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct ImportedMetric {
    pub kind: String,
    pub value: Option<i64>,
}

#[derive(Debug)]
pub struct ImportedDay {
    pub date: String,
    pub metrics: Vec<ImportedMetric>,
}

#[derive(Debug)]
pub struct ImportDailyMetricsCommand {
    pub user_id: UserId,
    pub provider: ProviderName,
    pub days: Vec<ImportedDay>,
    pub maximum_days: usize,
}
