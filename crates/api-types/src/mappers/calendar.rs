use application::entry::use_cases::get_calendar::CalendarDay;

use crate::responses::{CalendarDayResponse, EntryResponse};

impl From<CalendarDay> for CalendarDayResponse {
    fn from(day: CalendarDay) -> Self {
        Self {
            date: day.date,
            dominant_mood: day.dominant_mood.map(|m| m.value()),
            dominant_mood_label: day.dominant_mood.map(|m| format!("{m:?}")),
            entries: day.entries.into_iter().map(EntryResponse::from).collect(),
        }
    }
}
