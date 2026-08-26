use chrono::{DateTime, Utc};

use crate::user::UserId;

use super::{RejectionDetail, RejectionId, RejectionOrigin};

pub struct RejectedMetricData {
    pub id: RejectionId,
    pub user_id: UserId,
    pub origin: RejectionOrigin,
    pub detail: RejectionDetail,
    pub reason: String,
    pub recorded_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct RejectedMetric {
    id: RejectionId,
    user_id: UserId,
    origin: RejectionOrigin,
    detail: RejectionDetail,
    reason: String,
    recorded_at: DateTime<Utc>,
}

impl RejectedMetric {
    pub fn new(
        user_id: UserId,
        origin: RejectionOrigin,
        detail: RejectionDetail,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            id: RejectionId::generate(),
            user_id,
            origin,
            detail,
            reason: reason.into(),
            recorded_at: Utc::now(),
        }
    }

    pub fn from_persistence(data: RejectedMetricData) -> Self {
        Self {
            id: data.id,
            user_id: data.user_id,
            origin: data.origin,
            detail: data.detail,
            reason: data.reason,
            recorded_at: data.recorded_at,
        }
    }
}

impl RejectedMetric {
    pub fn id(&self) -> &RejectionId {
        &self.id
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn origin(&self) -> RejectionOrigin {
        self.origin
    }

    pub fn detail(&self) -> &RejectionDetail {
        &self.detail
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn recorded_at(&self) -> &DateTime<Utc> {
        &self.recorded_at
    }
}
