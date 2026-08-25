use chrono::{DateTime, FixedOffset};

use domain::activity::ActivityId;
use domain::attachment::{PhotoId, VoiceMemoId};
use domain::entry::{Content, Mood, MoodEntryId};
use domain::user::UserId;

#[derive(Debug)]
pub struct CreateEntryCommand {
    pub user_id: UserId,
    pub mood: Mood,
    pub logged_at: DateTime<FixedOffset>,
    pub activities: Vec<ActivityId>,
    pub content: Option<Content>,
    pub photos: Vec<PhotoId>,
    pub voice_memos: Vec<VoiceMemoId>,
}

#[derive(Debug)]
pub struct UpdateEntryCommand {
    pub entry_id: MoodEntryId,
    pub mood: Mood,
    pub logged_at: DateTime<FixedOffset>,
    pub activities: Vec<ActivityId>,
    pub content: Option<Content>,
    pub photos: Vec<PhotoId>,
    pub voice_memos: Vec<VoiceMemoId>,
}
