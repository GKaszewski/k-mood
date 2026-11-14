use std::collections::{BTreeMap, BTreeSet};

use domain::activity::ActivityId;
use domain::comparison::PeriodDay;
use domain::cycle::CycleCalendar;
use domain::dimension::ComposedEntry;
use domain::entry::{Date, DayMood};
use domain::song::Song;
use domain::user::Timezone;
use domain::weather::Weather;

use crate::day::date_of;

#[derive(Default)]
struct DayContents {
    entries: usize,
    activities: BTreeSet<ActivityId>,
    weather: Vec<Weather>,
    songs: Vec<Song>,
}

pub fn period_days(
    moods: &BTreeMap<Date, DayMood>,
    composed: &[ComposedEntry],
    timezone: &Timezone,
    calendar: Option<&CycleCalendar>,
) -> Vec<PeriodDay> {
    let mut contents: BTreeMap<Date, DayContents> = BTreeMap::new();

    for entry in composed {
        let day = contents.entry(date_of(&entry.entry, timezone)).or_default();
        day.entries += 1;
        day.activities.extend(entry.activities().iter().cloned());
        day.weather.extend(entry.weather().cloned());
        day.songs.extend(entry.song().cloned());
    }

    moods
        .iter()
        .map(|(date, mood)| {
            let day = contents.remove(date).unwrap_or_default();

            PeriodDay {
                date: *date,
                mood: *mood,
                entries: day.entries,
                activities: day.activities,
                weather: day.weather,
                songs: day.songs,
                cycle: calendar.and_then(|calendar| calendar.position_on(date)),
            }
        })
        .collect()
}
