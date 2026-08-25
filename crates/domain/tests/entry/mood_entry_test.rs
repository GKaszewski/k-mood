use chrono::{FixedOffset, TimeZone, Utc};

use domain::activity::ActivityId;
use domain::attachment::{PhotoId, VoiceMemoId};
use domain::entry::{Content, Mood, MoodEntry, MoodEntryData};
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
    assert!(entry.activities().is_empty());
    assert!(entry.content().is_none());
    assert!(entry.photos().is_empty());
    assert!(entry.voice_memos().is_empty());
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
fn set_activities_deduplicates_and_sorts() {
    let mut entry = MoodEntry::new(UserId::generate(), Mood::Meh, sample_logged_at());
    let a = ActivityId::generate();
    let b = ActivityId::generate();

    entry.set_activities(vec![b.clone(), a.clone(), b.clone()]);

    assert_eq!(entry.activities().len(), 2);
    let ids = entry.activities();
    assert!(ids[0] <= ids[1]);
}

#[test]
fn set_content_updates_and_clears() {
    let mut entry = MoodEntry::new(UserId::generate(), Mood::Good, sample_logged_at());

    let content = Content::new("great day").unwrap();
    entry.set_content(Some(content));
    assert!(entry.content().is_some());

    entry.set_content(None);
    assert!(entry.content().is_none());
}

#[test]
fn from_persistence_reconstructs_all_fields() {
    let id = domain::entry::MoodEntryId::generate();
    let user_id = UserId::generate();
    let activity_id = ActivityId::generate();
    let photo_id = PhotoId::generate();
    let voice_memo_id = VoiceMemoId::generate();
    let content = Content::new("test").unwrap();
    let now = Utc::now();

    let entry = MoodEntry::from_persistence(MoodEntryData {
        id: id.clone(),
        user_id: user_id.clone(),
        mood: Mood::Bad,
        logged_at: sample_logged_at(),
        activities: vec![activity_id.clone()],
        content: Some(content),
        photos: vec![photo_id.clone()],
        voice_memos: vec![voice_memo_id.clone()],
        created_at: now,
        updated_at: now,
    });

    assert_eq!(*entry.id(), id);
    assert_eq!(*entry.user_id(), user_id);
    assert_eq!(entry.mood(), Mood::Bad);
    assert_eq!(entry.activities().len(), 1);
    assert!(entry.content().is_some());
    assert_eq!(entry.photos().len(), 1);
    assert_eq!(entry.voice_memos().len(), 1);
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

#[test]
fn set_photos_replaces_list() {
    let mut entry = MoodEntry::new(UserId::generate(), Mood::Good, sample_logged_at());

    let p1 = PhotoId::generate();
    let p2 = PhotoId::generate();
    entry.set_photos(vec![p1.clone(), p2.clone()]);
    assert_eq!(entry.photos().len(), 2);

    entry.set_photos(vec![p1.clone()]);
    assert_eq!(entry.photos().len(), 1);

    entry.set_photos(vec![]);
    assert!(entry.photos().is_empty());
}

#[test]
fn set_voice_memos_replaces_list() {
    let mut entry = MoodEntry::new(UserId::generate(), Mood::Good, sample_logged_at());

    let m1 = VoiceMemoId::generate();
    let m2 = VoiceMemoId::generate();
    entry.set_voice_memos(vec![m1.clone(), m2.clone()]);
    assert_eq!(entry.voice_memos().len(), 2);

    entry.set_voice_memos(vec![]);
    assert!(entry.voice_memos().is_empty());
}
