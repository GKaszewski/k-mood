use chrono::{Duration, FixedOffset, TimeZone};

use domain::activity::ActivityId;
use domain::entry::{Mood, MoodEntry};
use domain::services::MoodAnalyzerService;
use domain::user::UserId;

fn entry_with_mood(mood: Mood, days_ago: i64) -> MoodEntry {
    let offset = FixedOffset::east_opt(0).unwrap();
    let logged_at =
        offset.from_utc_datetime(&(chrono::Utc::now() - Duration::days(days_ago)).naive_utc());
    MoodEntry::new(UserId::generate(), mood, logged_at)
}

fn entry_with_mood_and_activity(mood: Mood, activity: &ActivityId) -> MoodEntry {
    let offset = FixedOffset::east_opt(0).unwrap();
    let logged_at = offset.from_utc_datetime(&chrono::Utc::now().naive_utc());
    let mut entry = MoodEntry::new(UserId::generate(), mood, logged_at);
    entry.set_activities(vec![activity.clone()]);
    entry
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
    let entries = vec![
        entry_with_mood(Mood::Good, 0),
        entry_with_mood(Mood::Good, 1),
        entry_with_mood(Mood::Good, 2),
    ];

    assert_eq!(MoodAnalyzerService::current_streak(&entries), 3);
}

#[test]
fn streak_breaks_on_gap() {
    let entries = vec![
        entry_with_mood(Mood::Good, 0),
        entry_with_mood(Mood::Good, 1),
        entry_with_mood(Mood::Good, 3),
    ];

    assert_eq!(MoodAnalyzerService::current_streak(&entries), 2);
}

#[test]
fn streak_of_empty_entries_is_zero() {
    assert_eq!(MoodAnalyzerService::current_streak(&[]), 0);
}

#[test]
fn activity_correlation_positive_when_mood_higher_with_activity() {
    let activity = ActivityId::generate();
    let entries = vec![
        entry_with_mood_and_activity(Mood::Rad, &activity),
        entry_with_mood_and_activity(Mood::Good, &activity),
        entry_with_mood(Mood::Meh, 0),
        entry_with_mood(Mood::Bad, 1),
    ];

    let correlation = MoodAnalyzerService::activity_mood_correlation(&entries, &activity).unwrap();
    assert!(correlation > 0.0);
}

#[test]
fn activity_correlation_is_none_when_never_used() {
    let activity = ActivityId::generate();
    let entries = vec![entry_with_mood(Mood::Good, 0)];

    let result = MoodAnalyzerService::activity_mood_correlation(&entries, &activity);
    assert!(result.is_none());
}

#[test]
fn streak_is_zero_when_last_entry_is_old() {
    let entries = vec![
        entry_with_mood(Mood::Good, 3),
        entry_with_mood(Mood::Good, 4),
    ];
    assert_eq!(MoodAnalyzerService::current_streak(&entries), 0);
}

#[test]
fn mood_frequency_on_empty_is_empty() {
    let freq = MoodAnalyzerService::mood_frequency(&[]);
    assert!(freq.is_empty());
}

#[test]
fn activity_correlation_is_none_when_all_entries_have_activity() {
    let activity = ActivityId::generate();
    let entries = vec![
        entry_with_mood_and_activity(Mood::Good, &activity),
        entry_with_mood_and_activity(Mood::Rad, &activity),
    ];

    let result = MoodAnalyzerService::activity_mood_correlation(&entries, &activity);
    assert!(result.is_none());
}
