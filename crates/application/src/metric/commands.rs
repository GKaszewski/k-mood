use domain::entry::Date;
use domain::metric::{MetricKind, MetricValue, Source};
use domain::user::UserId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricChange {
    Stated(MetricValue),
    Cleared(MetricKind),
}

impl MetricChange {
    pub fn kind(&self) -> MetricKind {
        match self {
            Self::Stated(value) => value.kind(),
            Self::Cleared(kind) => *kind,
        }
    }
}

#[derive(Debug)]
pub struct SetDailyMetricsCommand {
    pub user_id: UserId,
    pub date: Date,
    pub changes: Vec<MetricChange>,
    pub source: Source,
}
