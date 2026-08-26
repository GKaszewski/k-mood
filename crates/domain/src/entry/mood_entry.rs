use chrono::{DateTime, FixedOffset, Utc};

use crate::user::UserId;

use super::{Mood, MoodEntryId};

pub struct MoodEntryData {
    pub id: MoodEntryId,
    pub user_id: UserId,
    pub mood: Mood,
    pub logged_at: DateTime<FixedOffset>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct MoodEntry {
    id: MoodEntryId,
    user_id: UserId,
    mood: Mood,
    logged_at: DateTime<FixedOffset>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl MoodEntry {
    pub fn new(user_id: UserId, mood: Mood, logged_at: DateTime<FixedOffset>) -> Self {
        let now = Utc::now();
        Self {
            id: MoodEntryId::generate(),
            user_id,
            mood,
            logged_at,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn from_persistence(data: MoodEntryData) -> Self {
        Self {
            id: data.id,
            user_id: data.user_id,
            mood: data.mood,
            logged_at: data.logged_at,
            created_at: data.created_at,
            updated_at: data.updated_at,
        }
    }
}

impl MoodEntry {
    pub fn id(&self) -> &MoodEntryId {
        &self.id
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn mood(&self) -> Mood {
        self.mood
    }

    pub fn logged_at(&self) -> &DateTime<FixedOffset> {
        &self.logged_at
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

impl MoodEntry {
    pub fn update_mood(&mut self, mood: Mood) {
        self.mood = mood;
        self.touch();
    }

    pub fn update_logged_at(&mut self, logged_at: DateTime<FixedOffset>) {
        self.logged_at = logged_at;
        self.touch();
    }

    fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}
