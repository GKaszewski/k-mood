use std::collections::{BTreeMap, BTreeSet};

use chrono::NaiveDate;

use domain::activity::ActivityId;
use domain::comparison::{PeriodDay, PeriodInput};
use domain::entry::{Date, DateSpan, DayMood, Mood};
use domain::metric::MetricKind;
use domain::provider::ProviderName;
use domain::song::Song;
use domain::weather::{Celsius, Condition, Weather};

pub const DEFAULT_TOP_MUSIC_COUNT: usize = 10;

pub fn date(text: &str) -> Date {
    Date::from_persistence(NaiveDate::parse_from_str(text, "%Y-%m-%d").unwrap())
}

pub fn october_day(day_of_month: u32) -> Date {
    Date::from_persistence(NaiveDate::from_ymd_opt(2026, 10, day_of_month).unwrap())
}

pub fn mood_of(value: u8) -> Mood {
    Mood::try_from(value).unwrap()
}

pub fn day_on(day_of_month: u32, mood_value: u8) -> PeriodDay {
    PeriodDay {
        date: october_day(day_of_month),
        mood: DayMood::of(&[mood_of(mood_value)]).unwrap(),
        entries: 1,
        activities: BTreeSet::new(),
        weather: Vec::new(),
        songs: Vec::new(),
        cycle: None,
    }
}

pub fn days_on(days_and_moods: &[(u32, u8)]) -> Vec<PeriodDay> {
    days_and_moods
        .iter()
        .map(|(day, mood)| day_on(*day, *mood))
        .collect()
}

pub fn weather(condition: Condition, degrees: f64) -> Weather {
    Weather::new(
        condition,
        Celsius::from_persistence(degrees),
        ProviderName::from_persistence("test".into()),
    )
}

pub fn song(title: &str, artist: &str) -> Song {
    Song::new(title, artist, None, None).unwrap()
}

pub fn activity_set(activity: &ActivityId) -> BTreeSet<ActivityId> {
    BTreeSet::from([activity.clone()])
}

pub fn october_input(days: Vec<PeriodDay>) -> PeriodInput {
    input_over("2026-10-01", "2026-10-31", days, BTreeMap::new())
}

pub fn input_over(
    start: &str,
    end: &str,
    days: Vec<PeriodDay>,
    metrics: BTreeMap<MetricKind, BTreeMap<Date, f64>>,
) -> PeriodInput {
    let span = DateSpan::new(date(start), date(end))
        .unwrap()
        .clipped_to(date("2026-11-14"))
        .unwrap();
    PeriodInput {
        span,
        days,
        metrics,
        tracks_cycle: false,
        top_music_count: DEFAULT_TOP_MUSIC_COUNT,
    }
}
