use std::sync::Arc;

use domain::dimension::{DimensionKind, DimensionValue};
use domain::entry::Mood;
use domain::ports::EntryDimensionPort;
use domain::testing::{InMemoryDimensionStore, InMemoryStore, test_logged_at, test_user};

use application::entry::commands::CreateEntryCommand;
use application::entry::use_cases::create_entry;

fn deps() -> (Arc<InMemoryStore>, create_entry::Deps) {
    let store = Arc::new(InMemoryStore::new());
    let content: Arc<dyn EntryDimensionPort> =
        Arc::new(InMemoryDimensionStore::new(DimensionKind::Content));
    let activities: Arc<dyn EntryDimensionPort> =
        Arc::new(InMemoryDimensionStore::new(DimensionKind::Activities));

    let deps = create_entry::Deps {
        activities: store.clone(),
        entries: store.clone(),
        dimensions: vec![content, activities],
        events: store.clone(),
    };
    (store, deps)
}

#[tokio::test]
async fn creates_entry_with_required_fields() {
    let (store, deps) = deps();
    let user = test_user("alice");

    let cmd = CreateEntryCommand {
        user_id: user.id().clone(),
        mood: Mood::Good,
        logged_at: test_logged_at(),
        dimensions: vec![],
    };

    let entry = create_entry::execute(cmd, &deps).await.unwrap();

    assert_eq!(entry.mood(), Mood::Good);
    assert_eq!(store.entry_count(), 1);
    assert_eq!(store.published_events().len(), 1);
}

#[tokio::test]
async fn creates_entry_with_optional_dimensions() {
    let (store, deps) = deps();
    let user = test_user("bob");
    let activity = domain::testing::test_activity(user.id().clone(), "gaming");
    domain::ports::ActivityCommandPort::save(store.as_ref(), &activity)
        .await
        .unwrap();

    let cmd = CreateEntryCommand {
        user_id: user.id().clone(),
        mood: Mood::Rad,
        logged_at: test_logged_at(),
        dimensions: vec![
            DimensionValue::Content(domain::testing::test_content("great day")),
            DimensionValue::activities(vec![activity.id().clone()]),
        ],
    };

    let entry = create_entry::execute(cmd, &deps).await.unwrap();

    assert_eq!(entry.mood(), Mood::Rad);
    assert_eq!(store.entry_count(), 1);
}

#[tokio::test]
async fn an_entry_cannot_wear_another_accounts_activity() {
    let (store, deps) = deps();
    let user = test_user("alice");
    let stranger = test_user("mallory");

    let theirs = domain::testing::test_activity(stranger.id().clone(), "fencing");
    domain::ports::ActivityCommandPort::save(store.as_ref(), &theirs)
        .await
        .unwrap();

    let cmd = CreateEntryCommand {
        user_id: user.id().clone(),
        mood: Mood::Good,
        logged_at: test_logged_at(),
        dimensions: vec![DimensionValue::activities(vec![theirs.id().clone()])],
    };

    let refused = create_entry::execute(cmd, &deps).await;

    assert!(
        matches!(
            refused,
            Err(application::errors::ApplicationError::Domain(
                domain::errors::DomainError::Forbidden(_)
            ))
        ),
        "tagging a stranger's activity must be refused, got {refused:?}"
    );
    assert_eq!(store.entry_count(), 0, "nothing should have been written");
}

#[tokio::test]
async fn an_entry_cannot_wear_an_activity_that_does_not_exist() {
    let (store, deps) = deps();
    let user = test_user("alice");

    let cmd = CreateEntryCommand {
        user_id: user.id().clone(),
        mood: Mood::Good,
        logged_at: test_logged_at(),
        dimensions: vec![DimensionValue::activities(vec![
            domain::activity::ActivityId::generate(),
        ])],
    };

    assert!(create_entry::execute(cmd, &deps).await.is_err());
    assert_eq!(store.entry_count(), 0);
}

#[tokio::test]
async fn an_archived_activity_cannot_be_tagged_onto_a_new_entry() {
    let (store, deps) = deps();
    let user = test_user("alice");

    let mut retired = domain::testing::test_activity(user.id().clone(), "fencing");
    retired.archive().unwrap();
    domain::ports::ActivityCommandPort::save(store.as_ref(), &retired)
        .await
        .unwrap();

    let cmd = CreateEntryCommand {
        user_id: user.id().clone(),
        mood: Mood::Good,
        logged_at: test_logged_at(),
        dimensions: vec![DimensionValue::activities(vec![retired.id().clone()])],
    };

    let refused = create_entry::execute(cmd, &deps).await;

    assert!(
        matches!(
            refused,
            Err(application::errors::ApplicationError::Validation(_))
        ),
        "an archived activity is retired, not forbidden, got {refused:?}"
    );
    assert_eq!(store.entry_count(), 0);
}

#[tokio::test]
async fn unarchiving_makes_an_activity_taggable_again() {
    let (store, deps) = deps();
    let user = test_user("alice");

    let mut activity = domain::testing::test_activity(user.id().clone(), "fencing");
    activity.archive().unwrap();
    activity.unarchive().unwrap();
    domain::ports::ActivityCommandPort::save(store.as_ref(), &activity)
        .await
        .unwrap();

    let cmd = CreateEntryCommand {
        user_id: user.id().clone(),
        mood: Mood::Good,
        logged_at: test_logged_at(),
        dimensions: vec![DimensionValue::activities(vec![activity.id().clone()])],
    };

    create_entry::execute(cmd, &deps).await.unwrap();

    assert_eq!(store.entry_count(), 1);
}

#[tokio::test]
async fn many_entries_land_in_one_request() {
    let store = Arc::new(InMemoryStore::new());
    let content: Arc<dyn EntryDimensionPort> =
        Arc::new(InMemoryDimensionStore::new(DimensionKind::Content));
    let user = test_user("alice");

    let deps = application::entry::use_cases::create_entries::Deps {
        entries: store.clone(),
        dimensions: vec![content],
        activities: store.clone(),
        events: store.clone(),
    };

    let wanted: Vec<CreateEntryCommand> = [Mood::Good, Mood::Bad, Mood::Rad]
        .into_iter()
        .map(|mood| CreateEntryCommand {
            user_id: user.id().clone(),
            mood,
            logged_at: test_logged_at(),
            dimensions: vec![],
        })
        .collect();

    let created =
        application::entry::use_cases::create_entries::execute(user.id().clone(), wanted, &deps)
            .await
            .unwrap();

    assert_eq!(created.len(), 3);
    assert_eq!(store.entry_count(), 3);
    assert_eq!(
        store.published_events().len(),
        3,
        "each entry is its own event"
    );
}

#[tokio::test]
async fn one_bad_activity_refuses_the_whole_batch() {
    let store = Arc::new(InMemoryStore::new());
    let user = test_user("alice");
    let stranger = test_user("mallory");

    let theirs = domain::testing::test_activity(stranger.id().clone(), "fencing");
    domain::ports::ActivityCommandPort::save(store.as_ref(), &theirs)
        .await
        .unwrap();

    let deps = application::entry::use_cases::create_entries::Deps {
        entries: store.clone(),
        dimensions: vec![],
        activities: store.clone(),
        events: store.clone(),
    };

    let wanted = vec![
        CreateEntryCommand {
            user_id: user.id().clone(),
            mood: Mood::Good,
            logged_at: test_logged_at(),
            dimensions: vec![],
        },
        CreateEntryCommand {
            user_id: user.id().clone(),
            mood: Mood::Bad,
            logged_at: test_logged_at(),
            dimensions: vec![DimensionValue::activities(vec![theirs.id().clone()])],
        },
    ];

    let refused =
        application::entry::use_cases::create_entries::execute(user.id().clone(), wanted, &deps)
            .await;

    assert!(refused.is_err());
    assert_eq!(
        store.entry_count(),
        0,
        "the first entry must not survive a batch that was refused"
    );
}
