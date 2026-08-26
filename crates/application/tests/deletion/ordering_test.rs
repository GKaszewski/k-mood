use std::sync::Arc;

use domain::attachment::{PhotoId, VoiceMemoId};
use domain::dimension::{DimensionKind, DimensionValue};
use domain::entry::{DateRange, Mood, MoodEntry};
use domain::ports::{
    ActivityQueryPort, CascadeDeletePort, EntryDimensionPort, MoodEntryCommandPort,
    ReminderQueryPort, UserCommandPort,
};
use domain::testing::{FakeMediaStorage, InMemoryDimensionStore, InMemoryStore, test_user};
use domain::user::{Timezone, User};

use application::entry::use_cases::delete_entries_by_date_range;
use application::user::use_cases::{clear_data, delete_user};

struct Doomed {
    store: Arc<InMemoryStore>,
    photos: Arc<InMemoryDimensionStore>,
    voice_memos: Arc<InMemoryDimensionStore>,
    user: User,
    photo_id: PhotoId,
    memo_id: VoiceMemoId,
}

async fn an_entry_with_media(media: &Arc<FakeMediaStorage>) -> Doomed {
    let store = Arc::new(InMemoryStore::new());
    let photos = Arc::new(InMemoryDimensionStore::new(DimensionKind::Photos));
    let voice_memos = Arc::new(InMemoryDimensionStore::new(DimensionKind::VoiceMemos));

    store.cascades_to(&photos);
    store.cascades_to(&voice_memos);

    let mut user = test_user("alice");
    user.update_timezone(Some(Timezone::new("Europe/Warsaw").unwrap()));
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();

    let entry = MoodEntry::new(
        user.id().clone(),
        Mood::Good,
        chrono::DateTime::parse_from_rfc3339("2026-08-20T12:00:00+02:00").unwrap(),
    );
    MoodEntryCommandPort::save(store.as_ref(), &entry)
        .await
        .unwrap();

    let photo_id = PhotoId::generate();
    let memo_id = VoiceMemoId::generate();
    media.put_photo(&photo_id, b"a photograph");
    media.put_voice_memo(&memo_id, b"a recording");

    photos
        .save(
            entry.id(),
            &[DimensionValue::Photos(vec![photo_id.clone()])],
        )
        .await
        .unwrap();
    voice_memos
        .save(
            entry.id(),
            &[DimensionValue::VoiceMemos(vec![memo_id.clone()])],
        )
        .await
        .unwrap();

    Doomed {
        store,
        photos,
        voice_memos,
        user,
        photo_id,
        memo_id,
    }
}

impl Doomed {
    fn dimensions(&self) -> Vec<Arc<dyn EntryDimensionPort>> {
        vec![
            self.photos.clone() as Arc<dyn EntryDimensionPort>,
            self.voice_memos.clone() as Arc<dyn EntryDimensionPort>,
        ]
    }

    fn everything_ever() -> DateRange {
        DateRange::new(
            chrono::DateTime::parse_from_rfc3339("2000-01-01T00:00:00+00:00").unwrap(),
            chrono::DateTime::parse_from_rfc3339("2099-01-01T00:00:00+00:00").unwrap(),
        )
        .unwrap()
    }
}

#[tokio::test]
async fn deleting_a_date_range_takes_the_blobs_with_it() {
    let media = Arc::new(FakeMediaStorage::holding_nothing());
    let doomed = an_entry_with_media(&media).await;

    let deps = delete_entries_by_date_range::Deps {
        cascade: doomed.store.clone(),
        query: doomed.store.clone(),
        dimensions: doomed.dimensions(),
        media_storage: media.clone(),
    };

    let deleted = delete_entries_by_date_range::execute(
        doomed.user.id().clone(),
        &Doomed::everything_ever(),
        &deps,
    )
    .await
    .unwrap();

    assert_eq!(deleted, 1);
    assert!(
        !media.holds_photo(&doomed.photo_id),
        "the photo blob was orphaned: media must be resolved before the cascade clears the dimensions"
    );
    assert!(
        !media.holds_voice_memo(&doomed.memo_id),
        "the voice memo blob was orphaned"
    );
    assert_eq!(media.blobs_held(), 0);
}

#[tokio::test]
async fn the_cascade_really_does_clear_the_dimension_values() {
    let media = Arc::new(FakeMediaStorage::holding_nothing());
    let doomed = an_entry_with_media(&media).await;
    assert_eq!(doomed.store.entry_count(), 1);

    CascadeDeletePort::delete_entries_in_range(
        doomed.store.as_ref(),
        doomed.user.id(),
        &Doomed::everything_ever(),
    )
    .await
    .unwrap();

    assert_eq!(
        doomed.photos.holds_count(),
        0,
        "a fake that keeps dimension values is more forgiving than the database"
    );
    assert_eq!(doomed.voice_memos.holds_count(), 0);
}

#[tokio::test]
async fn clearing_a_users_data_takes_the_blobs_with_it() {
    let media = Arc::new(FakeMediaStorage::holding_nothing());
    let doomed = an_entry_with_media(&media).await;

    let deps = clear_data::Deps {
        entry_query: doomed.store.clone(),
        dimensions: doomed.dimensions(),
        cascade: doomed.store.clone(),
        media_storage: media.clone(),
    };

    clear_data::execute(doomed.user.id().clone(), &deps)
        .await
        .unwrap();

    assert_eq!(
        media.blobs_held(),
        0,
        "clear_data has the same ordering requirement as deleting a range"
    );
}

#[tokio::test]
async fn deleting_an_account_takes_the_blobs_with_it() {
    let media = Arc::new(FakeMediaStorage::holding_nothing());
    let doomed = an_entry_with_media(&media).await;

    let deps = delete_user::Deps {
        entry_query: doomed.store.clone(),
        dimensions: doomed.dimensions(),
        cascade: doomed.store.clone(),
        media_storage: media.clone(),
        user_query: doomed.store.clone(),
        events: doomed.store.clone(),
    };

    delete_user::execute(doomed.user.id().clone(), &deps)
        .await
        .unwrap();

    assert_eq!(
        media.blobs_held(),
        0,
        "deleting an account has the same ordering requirement"
    );
    let _ = ActivityQueryPort::find_by_user(doomed.store.as_ref(), doomed.user.id()).await;
    let _ = ReminderQueryPort::find_by_user(doomed.store.as_ref(), doomed.user.id()).await;
}

#[tokio::test]
async fn a_blob_store_that_refuses_to_delete_does_not_save_the_entries() {
    let media = Arc::new(FakeMediaStorage::refusing_to_delete());
    let doomed = an_entry_with_media(&media).await;

    let deps = delete_entries_by_date_range::Deps {
        cascade: doomed.store.clone(),
        query: doomed.store.clone(),
        dimensions: doomed.dimensions(),
        media_storage: media.clone(),
    };

    let deleted = delete_entries_by_date_range::execute(
        doomed.user.id().clone(),
        &Doomed::everything_ever(),
        &deps,
    )
    .await
    .unwrap();

    assert_eq!(
        deleted, 1,
        "a blob that will not go must not keep the entry alive"
    );
    assert_eq!(doomed.store.entry_count(), 0);
    assert_eq!(
        media.deletions_attempted().len(),
        2,
        "both blobs were attempted even though the first refused"
    );
    assert_eq!(
        media.blobs_held(),
        2,
        "and the blobs are still there, orphaned but reported"
    );
}
