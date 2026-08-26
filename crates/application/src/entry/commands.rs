use chrono::{DateTime, FixedOffset};

use domain::dimension::DimensionValue;
use domain::entry::{Mood, MoodEntryId};
use domain::user::UserId;

#[derive(Debug)]
pub struct CreateEntryCommand {
    pub user_id: UserId,
    pub mood: Mood,
    pub logged_at: DateTime<FixedOffset>,
    pub dimensions: Vec<DimensionValue>,
}

#[derive(Debug)]
pub struct UpdateEntryCommand {
    pub entry_id: MoodEntryId,
    pub mood: Mood,
    pub logged_at: Option<DateTime<FixedOffset>>,
    pub dimensions: Vec<DimensionValue>,
}
