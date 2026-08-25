use domain::activity::ActivityId;
use domain::entry::{DateRange, Mood};
use domain::user::UserId;

#[derive(Debug)]
pub struct ListEntriesQuery {
    pub user_id: UserId,
    pub range: Option<DateRange>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug)]
pub struct FilterByMoodQuery {
    pub user_id: UserId,
    pub mood: Mood,
}

#[derive(Debug)]
pub struct FilterByActivityQuery {
    pub user_id: UserId,
    pub activity_id: ActivityId,
}

#[derive(Debug)]
pub struct MoodStatsQuery {
    pub user_id: UserId,
    pub range: Option<DateRange>,
}
