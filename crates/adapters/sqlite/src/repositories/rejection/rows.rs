use domain::entry::Date;
use domain::provider::ProviderName;
use domain::rejection::{
    RejectedMetric, RejectedMetricData, RejectionDetail, RejectionId, RejectionOrigin,
};
use domain::user::UserId;

#[derive(sqlx::FromRow)]
pub struct RejectionRow {
    pub id: String,
    pub user_id: String,
    pub origin: String,
    pub provider: Option<String>,
    pub date: Option<String>,
    pub kind: String,
    pub value: Option<i64>,
    pub reason: String,
    pub recorded_at: String,
}

pub fn readable(row: &RejectionRow) -> Option<RejectedMetric> {
    let origin = RejectionOrigin::from_name(&row.origin);

    if origin.is_none() {
        tracing::warn!(
            rejection_id = %row.id,
            origin = %row.origin,
            "skipped a stored rejection this build cannot read"
        );
    }

    let date = match &row.date {
        None => None,
        Some(day) => Some(Date::from_persistence(day.parse().ok()?)),
    };

    Some(RejectedMetric::from_persistence(RejectedMetricData {
        id: RejectionId::from_uuid(row.id.parse().ok()?),
        user_id: UserId::from_uuid(row.user_id.parse().ok()?),
        origin: origin?,
        detail: RejectionDetail::new(
            row.provider.clone().map(ProviderName::from_persistence),
            date,
            row.kind.clone(),
            row.value,
        ),
        reason: row.reason.clone(),
        recorded_at: row.recorded_at.parse().ok()?,
    }))
}
