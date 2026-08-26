use crate::entry::Date;
use crate::user::UserId;

use super::{MetricKind, MetricValue, Source};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DailyMetric {
    user_id: UserId,
    date: Date,
    value: MetricValue,
    source: Source,
}

impl DailyMetric {
    pub fn new(user_id: UserId, date: Date, value: MetricValue, source: Source) -> Self {
        Self {
            user_id,
            date,
            value,
            source,
        }
    }
}

impl DailyMetric {
    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn date(&self) -> &Date {
        &self.date
    }

    pub fn value(&self) -> &MetricValue {
        &self.value
    }

    pub fn source(&self) -> &Source {
        &self.source
    }

    pub fn kind(&self) -> MetricKind {
        self.value.kind()
    }

    pub fn supersedes(&self, stored: &DailyMetric) -> bool {
        stored.source.is_superseded_by(&self.source)
    }
}
