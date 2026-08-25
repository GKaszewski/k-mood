use chrono::{DateTime, FixedOffset, Utc};

use crate::activity::ActivityId;
use crate::attachment::{PhotoId, VoiceMemoId};
use crate::user::UserId;

use super::{Content, Mood, MoodEntryId};

pub struct MoodEntryData {
    pub id: MoodEntryId,
    pub user_id: UserId,
    pub mood: Mood,
    pub logged_at: DateTime<FixedOffset>,
    pub activities: Vec<ActivityId>,
    pub content: Option<Content>,
    pub photos: Vec<PhotoId>,
    pub voice_memos: Vec<VoiceMemoId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct MoodEntry {
    id: MoodEntryId,
    user_id: UserId,
    mood: Mood,
    logged_at: DateTime<FixedOffset>,
    activities: Vec<ActivityId>,
    content: Option<Content>,
    photos: Vec<PhotoId>,
    voice_memos: Vec<VoiceMemoId>,
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
            activities: Vec::new(),
            content: None,
            photos: Vec::new(),
            voice_memos: Vec::new(),
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
            activities: data.activities,
            content: data.content,
            photos: data.photos,
            voice_memos: data.voice_memos,
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

    pub fn activities(&self) -> &[ActivityId] {
        &self.activities
    }

    pub fn content(&self) -> Option<&Content> {
        self.content.as_ref()
    }

    pub fn photos(&self) -> &[PhotoId] {
        &self.photos
    }

    pub fn voice_memos(&self) -> &[VoiceMemoId] {
        &self.voice_memos
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

    pub fn set_content(&mut self, content: Option<Content>) {
        self.content = content;
        self.touch();
    }

    pub fn set_activities(&mut self, mut activities: Vec<ActivityId>) {
        activities.sort();
        activities.dedup();
        self.activities = activities;
        self.touch();
    }

    pub fn set_photos(&mut self, photos: Vec<PhotoId>) {
        self.photos = photos;
        self.touch();
    }

    pub fn set_voice_memos(&mut self, voice_memos: Vec<VoiceMemoId>) {
        self.voice_memos = voice_memos;
        self.touch();
    }

    fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}
