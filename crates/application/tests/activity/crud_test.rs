use std::sync::Arc;

use domain::activity::{ActivityName, CategoryName};
use domain::ports::ActivityCommandPort;
use domain::testing::{InMemoryStore, test_activity, test_user};

use domain::user::UserId;

use application::activity::commands::{
    CreateActivityCommand, RenameActivityCommand, SetCategoryCommand,
};
use application::activity::use_cases::{
    archive_activity, create_activity, delete_activity, get_activity, list_activities,
    rename_activity, set_category,
};

fn store_and_events() -> Arc<InMemoryStore> {
    Arc::new(InMemoryStore::new())
}

#[tokio::test]
async fn creates_activity() {
    let store = store_and_events();
    let user = test_user("alice");

    let deps = create_activity::Deps {
        activities: store.clone(),
        events: store.clone(),
    };

    let cmd = CreateActivityCommand {
        user_id: user.id().clone(),
        name: ActivityName::new("gaming").unwrap(),
        category: Some(CategoryName::new("hobbies").unwrap()),
    };

    let activity = create_activity::execute(cmd, &deps).await.unwrap();

    assert_eq!(activity.name().value(), "gaming");
    assert_eq!(activity.category().map(|c| c.value()), Some("hobbies"));
}

#[tokio::test]
async fn renames_activity() {
    let store = store_and_events();
    let user = test_user("alice");
    let activity = test_activity(user.id().clone(), "gaming");
    store.save(&activity).await.unwrap();

    let deps = rename_activity::Deps {
        command: store.clone(),
        query: store.clone(),
        events: store.clone(),
    };

    let cmd = RenameActivityCommand {
        activity_id: activity.id().clone(),
        new_name: ActivityName::new("video games").unwrap(),
    };

    rename_activity::execute(cmd, user.id().clone(), &deps)
        .await
        .unwrap();

    let fetched = get_activity::execute(
        activity.id().clone(),
        user.id().clone(),
        &get_activity::Deps {
            query: store.clone(),
        },
    )
    .await
    .unwrap();

    assert_eq!(fetched.name().value(), "video games");
}

#[tokio::test]
async fn sets_category() {
    let store = store_and_events();
    let user = test_user("alice");
    let activity = test_activity(user.id().clone(), "gaming");
    store.save(&activity).await.unwrap();

    let deps = set_category::Deps {
        command: store.clone(),
        query: store.clone(),
    };

    let cmd = SetCategoryCommand {
        activity_id: activity.id().clone(),
        category: Some(CategoryName::new("hobbies").unwrap()),
    };

    set_category::execute(cmd, user.id().clone(), &deps)
        .await
        .unwrap();

    let fetched = get_activity::execute(
        activity.id().clone(),
        user.id().clone(),
        &get_activity::Deps {
            query: store.clone(),
        },
    )
    .await
    .unwrap();

    assert_eq!(fetched.category().map(|c| c.value()), Some("hobbies"));
}

#[tokio::test]
async fn archives_and_unarchives() {
    let store = store_and_events();
    let user = test_user("alice");
    let activity = test_activity(user.id().clone(), "gaming");
    store.save(&activity).await.unwrap();

    let deps = archive_activity::Deps {
        command: store.clone(),
        query: store.clone(),
        events: store.clone(),
    };

    archive_activity::archive(activity.id().clone(), user.id().clone(), &deps)
        .await
        .unwrap();

    let list_deps = list_activities::Deps {
        query: store.clone(),
    };
    let active = list_activities::active_only(user.id().clone(), &list_deps)
        .await
        .unwrap();
    assert!(active.is_empty());

    archive_activity::unarchive(activity.id().clone(), user.id().clone(), &deps)
        .await
        .unwrap();

    let active = list_activities::active_only(user.id().clone(), &list_deps)
        .await
        .unwrap();
    assert_eq!(active.len(), 1);
}

#[tokio::test]
async fn deletes_activity() {
    let store = store_and_events();
    let user = test_user("alice");
    let activity = test_activity(user.id().clone(), "gaming");
    store.save(&activity).await.unwrap();

    let deps = delete_activity::Deps {
        entries: store.clone(),
        command: store.clone(),
        query: store.clone(),
    };

    delete_activity::execute(activity.id().clone(), user.id().clone(), &deps)
        .await
        .unwrap();

    assert_eq!(store.activity_count(), 0);
}

#[tokio::test]
async fn rename_rejects_different_user() {
    let store = store_and_events();
    let user = test_user("alice");
    let activity = test_activity(user.id().clone(), "gaming");
    store.save(&activity).await.unwrap();

    let deps = rename_activity::Deps {
        command: store.clone(),
        query: store.clone(),
        events: store.clone(),
    };

    let cmd = RenameActivityCommand {
        activity_id: activity.id().clone(),
        new_name: ActivityName::new("hacking").unwrap(),
    };

    let result = rename_activity::execute(cmd, UserId::generate(), &deps).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn delete_rejects_different_user() {
    let store = store_and_events();
    let user = test_user("alice");
    let activity = test_activity(user.id().clone(), "gaming");
    store.save(&activity).await.unwrap();

    let deps = delete_activity::Deps {
        entries: store.clone(),
        command: store.clone(),
        query: store.clone(),
    };

    let result = delete_activity::execute(activity.id().clone(), UserId::generate(), &deps).await;
    assert!(result.is_err());
    assert_eq!(store.activity_count(), 1);
}

#[tokio::test]
async fn archive_rejects_different_user() {
    let store = store_and_events();
    let user = test_user("alice");
    let activity = test_activity(user.id().clone(), "gaming");
    store.save(&activity).await.unwrap();

    let deps = archive_activity::Deps {
        command: store.clone(),
        query: store.clone(),
        events: store.clone(),
    };

    let result = archive_activity::archive(activity.id().clone(), UserId::generate(), &deps).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn an_activity_entries_still_wear_cannot_be_deleted() {
    let store = store_and_events();
    let user = test_user("alice");
    let activity = test_activity(user.id().clone(), "gaming");
    store.save(&activity).await.unwrap();

    let entry = domain::testing::test_entry(user.id().clone(), domain::entry::Mood::Good);
    domain::ports::MoodEntryCommandPort::save(store.as_ref(), &entry)
        .await
        .unwrap();
    store.assign_activities(entry.id(), vec![activity.id().clone()]);

    let deps = delete_activity::Deps {
        entries: store.clone(),
        command: store.clone(),
        query: store.clone(),
    };

    let refused = delete_activity::execute(activity.id().clone(), user.id().clone(), &deps).await;

    assert!(
        matches!(
            refused,
            Err(application::errors::ApplicationError::Validation(_))
        ),
        "deleting a tagged activity would rewrite history, got {refused:?}"
    );
    assert_eq!(
        store.activity_count(),
        1,
        "the activity must survive so it can be archived instead"
    );
}

#[tokio::test]
async fn an_archived_activity_is_still_listable_so_it_can_be_unarchived() {
    let store = store_and_events();
    let user = test_user("alice");

    let kept = test_activity(user.id().clone(), "reading");
    store.save(&kept).await.unwrap();

    let mut retired = test_activity(user.id().clone(), "fencing");
    retired.archive().unwrap();
    store.save(&retired).await.unwrap();

    let deps = list_activities::Deps {
        query: store.clone(),
    };

    let offered = list_activities::active_only(user.id().clone(), &deps)
        .await
        .unwrap();
    assert_eq!(offered.len(), 1, "only the active one may be tagged");

    let everything = list_activities::all(user.id().clone(), &deps)
        .await
        .unwrap();
    assert_eq!(
        everything.len(),
        2,
        "archiving must not put an activity beyond reach"
    );
    assert!(
        everything.iter().any(|activity| activity.is_archived()),
        "the archived one is the point"
    );
}
