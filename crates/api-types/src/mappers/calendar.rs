use application::entry::use_cases::get_calendar::CalendarDay;

use crate::responses::{CalendarDayResponse, EntryResponse};

impl From<CalendarDay> for CalendarDayResponse {
    fn from(day: CalendarDay) -> Self {
        Self {
            date: day.date.value(),
            day_mood: day.day_mood.map(|mood| mood.value()),
            mood: day.day_mood.map(|mood| mood.rounded().value()),
            mood_label: day.day_mood.map(|mood| format!("{:?}", mood.rounded())),
            cycle_day: day.cycle_day.map(|cycle| cycle.value()),
            entries: day.entries.into_iter().map(EntryResponse::from).collect(),
        }
    }
}
