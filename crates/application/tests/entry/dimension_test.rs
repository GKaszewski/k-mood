use std::sync::Arc;

use domain::attachment::{PhotoId, VoiceMemoId};
use domain::dimension::{DimensionKind, DimensionValue};
use domain::entry::{Mood, MoodEntry};
use domain::ports::EntryDimensionPort;
use domain::testing::{InMemoryDimensionStore, test_content, test_logged_at, test_user};

use application::entry::commands::CreateEntryCommand;
use application::entry::composition::EntryComposer;
use application::entry::use_cases::create_entry;

struct Dimensions {
    content: Arc<InMemoryDimensionStore>,
    activities: Arc<InMemoryDimensionStore>,
    photos: Arc<InMemoryDimensionStore>,
    voice_memos: Arc<InMemoryDimensionStore>,
}

impl Dimensions {
    fn new() -> Self {
        Self {
            content: Arc::new(InMemoryDimensionStore::new(DimensionKind::Content)),
            activities: Arc::new(InMemoryDimensionStore::new(DimensionKind::Activities)),
            photos: Arc::new(InMemoryDimensionStore::new(DimensionKind::Photos)),
            voice_memos: Arc::new(InMemoryDimensionStore::new(DimensionKind::VoiceMemos)),
        }
    }

    fn ports(&self) -> Vec<Arc<dyn EntryDimensionPort>> {
        vec![
            self.content.clone(),
            self.activities.clone(),
            self.photos.clone(),
            self.voice_memos.clone(),
        ]
    }

    fn composer(&self) -> EntryComposer {
        EntryComposer::new(self.ports())
    }
}

#[tokio::test]
async fn content_saved_for_an_entry_is_returned_when_that_entry_is_composed() {
    let dimensions = Dimensions::new();
    let user = test_user("alice");
    let entry = MoodEntry::new(user.id().clone(), Mood::Good, test_logged_at());

    dimensions.content.put(
        entry.id(),
        DimensionValue::Content(test_content("went for a walk")),
    );

    let composed = dimensions.composer().compose(vec![entry]).await.unwrap();

    assert_eq!(
        composed[0].content().map(|c| c.value()),
        Some("went for a walk")
    );
}

#[tokio::test]
async fn an_entry_with_no_content_composes_with_no_dimensions() {
    let dimensions = Dimensions::new();
    let user = test_user("carol");
    let entry = MoodEntry::new(user.id().clone(), Mood::Meh, test_logged_at());

    let composed = dimensions.composer().compose(vec![entry]).await.unwrap();

    assert!(composed[0].dimensions.is_empty());
}

#[tokio::test]
async fn each_entry_in_a_page_composes_with_only_its_own_content() {
    let dimensions = Dimensions::new();
    let user = test_user("dave");

    let first = MoodEntry::new(user.id().clone(), Mood::Good, test_logged_at());
    let second = MoodEntry::new(user.id().clone(), Mood::Bad, test_logged_at());
    let third = MoodEntry::new(user.id().clone(), Mood::Rad, test_logged_at());

    dimensions
        .content
        .put(first.id(), DimensionValue::Content(test_content("mine")));
    dimensions
        .content
        .put(third.id(), DimensionValue::Content(test_content("theirs")));

    let composed = dimensions
        .composer()
        .compose(vec![first, second, third])
        .await
        .unwrap();

    assert_eq!(composed[0].content().map(|c| c.value()), Some("mine"));
    assert!(composed[1].dimensions.is_empty());
    assert_eq!(composed[2].content().map(|c| c.value()), Some("theirs"));
}

#[tokio::test]
async fn an_entry_composes_every_kind_of_dimension_it_has() {
    let dimensions = Dimensions::new();
    let store = Arc::new(domain::testing::InMemoryStore::new());
    let user = test_user("erin");

    let activity = domain::testing::test_activity(user.id().clone(), "reading");
    domain::ports::ActivityCommandPort::save(store.as_ref(), &activity)
        .await
        .unwrap();
    let activity_id = activity.id().clone();
    let photo_id = PhotoId::generate();
    let voice_memo_id = VoiceMemoId::generate();

    let deps = create_entry::Deps {
        activities: store.clone(),
        entries: store.clone(),
        dimensions: dimensions.ports(),
        events: store.clone(),
    };

    let cmd = CreateEntryCommand {
        user_id: user.id().clone(),
        mood: Mood::Rad,
        logged_at: test_logged_at(),
        dimensions: vec![
            DimensionValue::Content(test_content("a full day")),
            DimensionValue::Activities(vec![activity_id.clone()]),
            DimensionValue::Photos(vec![photo_id.clone()]),
            DimensionValue::VoiceMemos(vec![voice_memo_id.clone()]),
        ],
    };

    let entry = create_entry::execute(cmd, &deps).await.unwrap();

    let composed = dimensions.composer().compose(vec![entry]).await.unwrap();
    let composed = &composed[0];

    assert_eq!(composed.content().map(|c| c.value()), Some("a full day"));
    assert_eq!(composed.activities(), [activity_id]);
    assert_eq!(composed.photos(), [photo_id]);
    assert_eq!(composed.voice_memos(), [voice_memo_id]);
}
