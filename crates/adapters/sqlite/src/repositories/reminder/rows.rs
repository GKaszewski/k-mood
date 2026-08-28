use chrono::NaiveTime;

use domain::reminder::{DaySchedule, Reminder, ReminderId};
use domain::user::UserId;

#[derive(sqlx::FromRow)]
pub struct ReminderRow {
    pub id: String,
    pub user_id: String,
    pub monday: Option<String>,
    pub tuesday: Option<String>,
    pub wednesday: Option<String>,
    pub thursday: Option<String>,
    pub friday: Option<String>,
    pub saturday: Option<String>,
    pub sunday: Option<String>,
    pub enabled: bool,
    pub created_at: String,
    pub last_sent_at: Option<String>,
}

impl ReminderRow {
    pub fn into_domain(self) -> Reminder {
        let schedule = DaySchedule::from_persistence(
            self.monday.and_then(|s| parse_time(&s)),
            self.tuesday.and_then(|s| parse_time(&s)),
            self.wednesday.and_then(|s| parse_time(&s)),
            self.thursday.and_then(|s| parse_time(&s)),
            self.friday.and_then(|s| parse_time(&s)),
            self.saturday.and_then(|s| parse_time(&s)),
            self.sunday.and_then(|s| parse_time(&s)),
        );

        Reminder::from_persistence(
            ReminderId::from_uuid(self.id.parse().unwrap()),
            UserId::from_uuid(self.user_id.parse().unwrap()),
            schedule,
            self.enabled,
            self.created_at.parse().unwrap(),
            self.last_sent_at.and_then(|sent| sent.parse().ok()),
        )
    }
}

fn parse_time(s: &str) -> Option<NaiveTime> {
    NaiveTime::parse_from_str(s, "%H:%M").ok()
}

pub fn format_time(t: NaiveTime) -> String {
    t.format("%H:%M").to_string()
}
