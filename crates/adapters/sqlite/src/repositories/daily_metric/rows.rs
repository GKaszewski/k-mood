use domain::entry::Date;
use domain::metric::{DailyMetric, MetricKind, MetricValue, Source};
use domain::provider::ProviderName;
use domain::user::UserId;

#[derive(sqlx::FromRow)]
pub struct DailyMetricRow {
    pub user_id: String,
    pub date: String,
    pub kind: String,
    pub value: i64,
    pub provider: Option<String>,
}

pub fn row_to_metric(row: &DailyMetricRow) -> Option<DailyMetric> {
    let user_id = row.user_id.parse().ok().map(UserId::from_uuid)?;
    let date = row.date.parse().ok().map(Date::from_persistence)?;
    let kind = MetricKind::from_name(&row.kind)?;
    let value = MetricValue::of_kind(kind, row.value).ok()?;

    Some(DailyMetric::new(user_id, date, value, source_of(row)))
}

pub fn source_of(row: &DailyMetricRow) -> Source {
    match &row.provider {
        None => Source::Manual,
        Some(name) => Source::Provider(ProviderName::from_persistence(name.clone())),
    }
}

pub fn provider_column(source: &Source) -> Option<&str> {
    source.provider().map(|name| name.value())
}
