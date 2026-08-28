use chrono::{DateTime, Utc};

use crate::activity::ActivityId;
use crate::user::UserId;

use super::{DateRange, Mood};

#[derive(Debug, Clone)]
pub struct EntrySelection {
    user_id: UserId,
    logged_within: Option<DateRange>,
    mood: Option<Mood>,
    tagged_with: Option<ActivityId>,
    changed_since: Option<DateTime<Utc>>,
}

impl EntrySelection {
    pub fn everything_of(user_id: UserId) -> Self {
        Self {
            user_id,
            logged_within: None,
            mood: None,
            tagged_with: None,
            changed_since: None,
        }
    }

    pub fn logged_within(mut self, range: Option<DateRange>) -> Self {
        self.logged_within = range;
        self
    }

    pub fn of_mood(mut self, mood: Option<Mood>) -> Self {
        self.mood = mood;
        self
    }

    pub fn tagged_with(mut self, activity_id: Option<ActivityId>) -> Self {
        self.tagged_with = activity_id;
        self
    }

    pub fn changed_since(mut self, instant: Option<DateTime<Utc>>) -> Self {
        self.changed_since = instant;
        self
    }
}

impl EntrySelection {
    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn range(&self) -> Option<&DateRange> {
        self.logged_within.as_ref()
    }

    pub fn mood(&self) -> Option<Mood> {
        self.mood
    }

    pub fn activity(&self) -> Option<&ActivityId> {
        self.tagged_with.as_ref()
    }

    pub fn since(&self) -> Option<DateTime<Utc>> {
        self.changed_since
    }
}
