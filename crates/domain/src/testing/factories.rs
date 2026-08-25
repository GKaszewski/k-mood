use chrono::{DateTime, FixedOffset, NaiveTime, TimeZone, Utc};

use crate::activity::{Activity, ActivityName, CategoryName};
use crate::attachment::ContentType;
use crate::entry::{Content, DateRange, Mood, MoodEntry};
use crate::reminder::{DaySchedule, Reminder};
use crate::user::{Email, PasswordHash, User, UserId, Username};

pub fn test_user(name: &str) -> User {
    User::new(
        Username::new(name).unwrap(),
        Email::new(format!("{name}@example.com")).unwrap(),
        PasswordHash::new(format!("hashed:{name}")),
    )
}

pub fn test_entry(user_id: UserId, mood: Mood) -> MoodEntry {
    MoodEntry::new(user_id, mood, test_logged_at())
}

pub fn test_entry_days_ago(user_id: UserId, mood: Mood, days_ago: i64) -> MoodEntry {
    let offset = FixedOffset::east_opt(0).unwrap();
    let logged_at =
        offset.from_utc_datetime(&(Utc::now() - chrono::Duration::days(days_ago)).naive_utc());
    MoodEntry::new(user_id, mood, logged_at)
}

pub fn test_activity(user_id: UserId, name: &str) -> Activity {
    Activity::new(user_id, ActivityName::new(name).unwrap(), None)
}

pub fn test_activity_with_category(user_id: UserId, name: &str, category: &str) -> Activity {
    Activity::new(
        user_id,
        ActivityName::new(name).unwrap(),
        Some(CategoryName::new(category).unwrap()),
    )
}

pub fn test_reminder(user_id: UserId) -> Reminder {
    let time = NaiveTime::from_hms_opt(20, 0, 0).unwrap();
    Reminder::new(user_id, DaySchedule::every_day_at(time))
}

pub fn test_content(text: &str) -> Content {
    Content::new(text).unwrap()
}

pub fn test_content_type(mime: &str) -> ContentType {
    ContentType::new(mime).unwrap()
}

pub fn test_logged_at() -> DateTime<FixedOffset> {
    FixedOffset::east_opt(3600)
        .unwrap()
        .with_ymd_and_hms(2025, 6, 15, 20, 0, 0)
        .unwrap()
}

pub fn test_date_range_last_n_days(days: i64) -> DateRange {
    let offset = FixedOffset::east_opt(0).unwrap();
    let now = offset.from_utc_datetime(&Utc::now().naive_utc());
    let start = offset.from_utc_datetime(&(Utc::now() - chrono::Duration::days(days)).naive_utc());
    DateRange::new(start, now).unwrap()
}
