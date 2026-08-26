use chrono::{FixedOffset, TimeZone, Utc};

use domain::entry::{Mood, MoodEntry, MoodEntryData};
use domain::user::UserId;

fn sample_logged_at() -> chrono::DateTime<FixedOffset> {
    FixedOffset::east_opt(3600)
        .unwrap()
        .with_ymd_and_hms(2025, 3, 15, 20, 0, 0)
        .unwrap()
}

#[test]
fn new_entry_has_required_fields() {
    let user_id = UserId::generate();
    let entry = MoodEntry::new(user_id.clone(), Mood::Good, sample_logged_at());

    assert_eq!(*entry.user_id(), user_id);
    assert_eq!(entry.mood(), Mood::Good);
}

#[test]
fn update_mood_changes_value_and_touches_updated_at() {
    let mut entry = MoodEntry::new(UserId::generate(), Mood::Good, sample_logged_at());
    let before = *entry.updated_at();

    entry.update_mood(Mood::Rad);

    assert_eq!(entry.mood(), Mood::Rad);
    assert!(*entry.updated_at() >= before);
}

#[test]
fn from_persistence_reconstructs_all_fields() {
    let id = domain::entry::MoodEntryId::generate();
    let user_id = UserId::generate();
    let now = Utc::now();

    let entry = MoodEntry::from_persistence(MoodEntryData {
        id: id.clone(),
        user_id: user_id.clone(),
        mood: Mood::Bad,
        logged_at: sample_logged_at(),
        created_at: now,
        updated_at: now,
    });

    assert_eq!(*entry.id(), id);
    assert_eq!(*entry.user_id(), user_id);
    assert_eq!(entry.mood(), Mood::Bad);
    assert_eq!(*entry.logged_at(), sample_logged_at());
}

#[test]
fn update_logged_at_changes_timestamp() {
    let mut entry = MoodEntry::new(UserId::generate(), Mood::Good, sample_logged_at());

    let new_time = FixedOffset::east_opt(3600)
        .unwrap()
        .with_ymd_and_hms(2025, 6, 1, 12, 0, 0)
        .unwrap();
    entry.update_logged_at(new_time);

    assert_eq!(*entry.logged_at(), new_time);
}
