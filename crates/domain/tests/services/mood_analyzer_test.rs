use chrono::{Duration, FixedOffset, TimeZone};

use domain::entry::Date;
use domain::entry::{Mood, MoodEntry};
use domain::services::MoodAnalyzerService;
use domain::user::Timezone;
use domain::user::UserId;

fn entry_with_mood(mood: Mood, days_ago: i64) -> MoodEntry {
    let offset = FixedOffset::east_opt(0).unwrap();
    let logged_at =
        offset.from_utc_datetime(&(chrono::Utc::now() - Duration::days(days_ago)).naive_utc());
    MoodEntry::new(UserId::generate(), mood, logged_at)
}

#[test]
fn average_mood_of_empty_slice_is_none() {
    assert!(MoodAnalyzerService::average_mood(&[]).is_none());
}

#[test]
fn average_mood_of_single_entry() {
    let entries = vec![entry_with_mood(Mood::Good, 0)];
    let avg = MoodAnalyzerService::average_mood(&entries).unwrap();
    assert!((avg - 4.0).abs() < f64::EPSILON);
}

#[test]
fn average_mood_across_multiple_entries() {
    let entries = vec![
        entry_with_mood(Mood::Awful, 0),
        entry_with_mood(Mood::Rad, 0),
    ];
    let avg = MoodAnalyzerService::average_mood(&entries).unwrap();
    assert!((avg - 3.0).abs() < f64::EPSILON);
}

#[test]
fn mood_frequency_counts_each_mood() {
    let entries = vec![
        entry_with_mood(Mood::Good, 0),
        entry_with_mood(Mood::Good, 1),
        entry_with_mood(Mood::Meh, 2),
    ];

    let freq = MoodAnalyzerService::mood_frequency(&entries);

    let good_count = freq.iter().find(|(m, _)| *m == Mood::Good).map(|(_, c)| *c);
    let meh_count = freq.iter().find(|(m, _)| *m == Mood::Meh).map(|(_, c)| *c);

    assert_eq!(good_count, Some(2));
    assert_eq!(meh_count, Some(1));
}

#[test]
fn streak_counts_consecutive_days() {
    let dates = dates_ago(&[0, 1, 2]);

    assert_eq!(MoodAnalyzerService::current_streak(&dates, today()), 3);
}

#[test]
fn streak_breaks_on_gap() {
    let dates = dates_ago(&[0, 1, 3]);

    assert_eq!(MoodAnalyzerService::current_streak(&dates, today()), 2);
}

#[test]
fn streak_of_no_dates_is_zero() {
    assert_eq!(MoodAnalyzerService::current_streak(&[], today()), 0);
}

#[test]
fn streak_is_zero_when_the_last_entry_is_old() {
    let dates = dates_ago(&[3, 4]);

    assert_eq!(MoodAnalyzerService::current_streak(&dates, today()), 0);
}

#[test]
fn the_streak_is_measured_in_the_users_own_days_not_utc_days() {
    // 2025-03-15T20:00Z is already the 16th in Auckland and still the 15th in UTC.
    let evening: chrono::DateTime<FixedOffset> = "2025-03-15T20:00:00+00:00".parse().unwrap();
    let auckland = Timezone::new("Pacific/Auckland").unwrap();

    let logged = Date::from_instant(&evening, &auckland);
    let their_today = Date::from_instant(&evening, &auckland);

    assert_eq!(
        MoodAnalyzerService::current_streak(&[logged], their_today),
        1
    );
}

#[test]
fn mood_frequency_on_empty_is_empty() {
    let freq = MoodAnalyzerService::mood_frequency(&[]);
    assert!(freq.is_empty());
}

fn today() -> Date {
    Date::from_instant(
        &chrono::Utc::now().fixed_offset(),
        &Timezone::new("UTC").unwrap(),
    )
}

fn dates_ago(offsets: &[i64]) -> Vec<Date> {
    let utc = Timezone::new("UTC").unwrap();
    offsets
        .iter()
        .map(|days| {
            let instant = chrono::Utc::now() - chrono::Duration::days(*days);
            Date::from_instant(&instant.fixed_offset(), &utc)
        })
        .collect()
}
