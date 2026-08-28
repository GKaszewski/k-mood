use std::sync::Arc;

use domain::entry::{EntrySelection, Mood, Pagination};
use domain::ports::MoodEntryCommandPort;
use domain::testing::{InMemoryStore, test_activity, test_entry, test_user};
use domain::user::User;

use application::entry::queries::ListEntriesQuery;
use application::entry::use_cases::list_entries;

const A_GENEROUS_PAGE: i64 = 500;

fn everything_of(user: &User) -> ListEntriesQuery {
    ListEntriesQuery {
        selection: EntrySelection::everything_of(user.id().clone()),
        page: Pagination::new(A_GENEROUS_PAGE, 0, A_GENEROUS_PAGE).unwrap(),
    }
}

async fn logged(store: &Arc<InMemoryStore>, user: &User, mood: Mood) -> domain::entry::MoodEntry {
    let entry = test_entry(user.id().clone(), mood);
    store.save(&entry).await.unwrap();

    entry
}

#[tokio::test]
async fn lists_only_the_asking_account() {
    let store = Arc::new(InMemoryStore::new());
    let alice = test_user("alice");
    let bob = test_user("bob");

    logged(&store, &alice, Mood::Good).await;
    logged(&store, &alice, Mood::Rad).await;
    logged(&store, &bob, Mood::Meh).await;

    let deps = list_entries::Deps {
        query: store.clone(),
    };
    let page = list_entries::execute(everything_of(&alice), &deps)
        .await
        .unwrap();

    assert_eq!(page.items().len(), 2);
    assert_eq!(page.total(), 2);
}

#[tokio::test]
async fn an_account_with_nothing_logged_gets_an_empty_page() {
    let store = Arc::new(InMemoryStore::new());
    let deps = list_entries::Deps {
        query: store.clone(),
    };

    let page = list_entries::execute(everything_of(&test_user("nobody")), &deps)
        .await
        .unwrap();

    assert!(page.items().is_empty());
    assert_eq!(page.total(), 0);
    assert!(!page.more_after_this());
}

#[tokio::test]
async fn the_total_counts_the_selection_not_the_page() {
    let store = Arc::new(InMemoryStore::new());
    let alice = test_user("alice");

    for _ in 0..7 {
        logged(&store, &alice, Mood::Good).await;
    }

    let deps = list_entries::Deps {
        query: store.clone(),
    };
    let query = ListEntriesQuery {
        selection: EntrySelection::everything_of(alice.id().clone()),
        page: Pagination::new(3, 0, A_GENEROUS_PAGE).unwrap(),
    };

    let page = list_entries::execute(query, &deps).await.unwrap();

    assert_eq!(page.items().len(), 3, "the page holds what was asked for");
    assert_eq!(page.total(), 7, "the total is what a client pages through");
    assert!(page.more_after_this());
}

#[tokio::test]
async fn the_last_page_says_there_is_nothing_after_it() {
    let store = Arc::new(InMemoryStore::new());
    let alice = test_user("alice");

    for _ in 0..5 {
        logged(&store, &alice, Mood::Good).await;
    }

    let deps = list_entries::Deps {
        query: store.clone(),
    };
    let query = ListEntriesQuery {
        selection: EntrySelection::everything_of(alice.id().clone()),
        page: Pagination::new(3, 3, A_GENEROUS_PAGE).unwrap(),
    };

    let page = list_entries::execute(query, &deps).await.unwrap();

    assert_eq!(page.items().len(), 2);
    assert_eq!(page.total(), 5);
    assert!(!page.more_after_this());
}

#[tokio::test]
async fn a_selection_can_narrow_to_one_mood() {
    let store = Arc::new(InMemoryStore::new());
    let alice = test_user("alice");

    logged(&store, &alice, Mood::Good).await;
    logged(&store, &alice, Mood::Good).await;
    logged(&store, &alice, Mood::Bad).await;

    let deps = list_entries::Deps {
        query: store.clone(),
    };
    let query = ListEntriesQuery {
        selection: EntrySelection::everything_of(alice.id().clone()).of_mood(Some(Mood::Good)),
        page: Pagination::new(A_GENEROUS_PAGE, 0, A_GENEROUS_PAGE).unwrap(),
    };

    let page = list_entries::execute(query, &deps).await.unwrap();

    assert_eq!(page.items().len(), 2);
    assert_eq!(page.total(), 2);
}

#[tokio::test]
async fn a_selection_can_narrow_to_one_activity() {
    let store = Arc::new(InMemoryStore::new());
    let alice = test_user("alice");
    let exercise = test_activity(alice.id().clone(), "exercise");

    let tagged = logged(&store, &alice, Mood::Good).await;
    store.assign_activities(tagged.id(), vec![exercise.id().clone()]);
    logged(&store, &alice, Mood::Meh).await;

    let deps = list_entries::Deps {
        query: store.clone(),
    };
    let query = ListEntriesQuery {
        selection: EntrySelection::everything_of(alice.id().clone())
            .tagged_with(Some(exercise.id().clone())),
        page: Pagination::new(A_GENEROUS_PAGE, 0, A_GENEROUS_PAGE).unwrap(),
    };

    let page = list_entries::execute(query, &deps).await.unwrap();

    assert_eq!(page.items().len(), 1);
    assert_eq!(page.items()[0].id(), tagged.id());
}

#[tokio::test]
async fn a_page_bigger_than_the_server_serves_is_refused() {
    assert!(Pagination::new(1_000, 0, 200).is_err());
    assert!(Pagination::new(0, 0, 200).is_err());
    assert!(Pagination::new(10, -1, 200).is_err());
}
