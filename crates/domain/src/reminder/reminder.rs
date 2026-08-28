use chrono::{DateTime, Duration, Utc};

use crate::user::{Timezone, UserId};

use super::due_occurrence::{DueOccurrence, occurrence_reached};
use super::{DaySchedule, ReminderId};

#[derive(Debug, Clone)]
pub struct Reminder {
    id: ReminderId,
    user_id: UserId,
    schedule: DaySchedule,
    enabled: bool,
    created_at: DateTime<Utc>,
    last_sent_at: Option<DateTime<Utc>>,
}

impl Reminder {
    pub fn new(user_id: UserId, schedule: DaySchedule) -> Self {
        Self {
            id: ReminderId::generate(),
            user_id,
            schedule,
            enabled: true,
            created_at: Utc::now(),
            last_sent_at: None,
        }
    }

    pub fn from_persistence(
        id: ReminderId,
        user_id: UserId,
        schedule: DaySchedule,
        enabled: bool,
        created_at: DateTime<Utc>,
        last_sent_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            user_id,
            schedule,
            enabled,
            created_at,
            last_sent_at,
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

    pub fn last_sent_at(&self) -> Option<DateTime<Utc>> {
        self.last_sent_at
    }

    pub fn occurrence_reached(
        &self,
        now: DateTime<Utc>,
        timezone: &Timezone,
        grace: Duration,
    ) -> Option<DueOccurrence> {
        if !self.enabled {
            return None;
        }

        occurrence_reached(&self.schedule, self.last_sent_at, now, timezone, grace)
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

    pub fn mark_sent(&mut self, occurrence: DueOccurrence) {
        self.last_sent_at = Some(occurrence.scheduled_for());
    }
}
