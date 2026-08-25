use chrono::{DateTime, Utc};

use crate::user::UserId;

use super::{DaySchedule, ReminderId};

#[derive(Debug, Clone)]
pub struct Reminder {
    id: ReminderId,
    user_id: UserId,
    schedule: DaySchedule,
    enabled: bool,
    created_at: DateTime<Utc>,
}

impl Reminder {
    pub fn new(user_id: UserId, schedule: DaySchedule) -> Self {
        Self {
            id: ReminderId::generate(),
            user_id,
            schedule,
            enabled: true,
            created_at: Utc::now(),
        }
    }

    pub fn from_persistence(
        id: ReminderId,
        user_id: UserId,
        schedule: DaySchedule,
        enabled: bool,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            user_id,
            schedule,
            enabled,
            created_at,
        }
    }
}

impl Reminder {
    pub fn id(&self) -> &ReminderId {
        &self.id
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn schedule(&self) -> &DaySchedule {
        &self.schedule
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}

impl Reminder {
    pub fn update_schedule(&mut self, schedule: DaySchedule) {
        self.schedule = schedule;
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
}
