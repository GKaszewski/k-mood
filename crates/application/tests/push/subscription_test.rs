use std::sync::Arc;

use domain::ports::PushSubscriptionQueryPort;
use domain::testing::{InMemoryStore, test_user};

use application::push::commands::{SubscribePushCommand, UnsubscribePushCommand};
use application::push::use_cases::{subscribe, unsubscribe};

const ENDPOINT: &str = "https://push.example.com/a-device";

fn a_subscription(user_id: domain::user::UserId) -> SubscribePushCommand {
    SubscribePushCommand {
        user_id,
        endpoint: ENDPOINT.into(),
        p256dh: "a-key".into(),
        auth: "a-secret".into(),
    }
}

#[tokio::test]
async fn subscribing_twice_keeps_one_subscription() {
    let store = Arc::new(InMemoryStore::new());
    let user = test_user("alice");

    let deps = subscribe::Deps {
        push_command: store.clone(),
        push_query: store.clone(),
    };

    subscribe::execute(a_subscription(user.id().clone()), &deps)
        .await
        .unwrap();
    let first = store.find_by_endpoint(ENDPOINT).await.unwrap().unwrap();

    subscribe::execute(a_subscription(user.id().clone()), &deps)
        .await
        .unwrap();

    let held = store.find_by_user(user.id()).await.unwrap();

    assert_eq!(held.len(), 1, "one device is one subscription");
    assert_eq!(
        held[0].id(),
        first.id(),
        "re-subscribing renews the subscription rather than minting a new one"
    );
}

#[tokio::test]
async fn a_stranger_cannot_unsubscribe_someone_elses_device() {
    let store = Arc::new(InMemoryStore::new());
    let owner = test_user("alice");
    let stranger = test_user("mallory");

    let deps = subscribe::Deps {
        push_command: store.clone(),
        push_query: store.clone(),
    };
    subscribe::execute(a_subscription(owner.id().clone()), &deps)
        .await
        .unwrap();

    let deps = unsubscribe::Deps {
        push_command: store.clone(),
    };

    unsubscribe::execute(
        UnsubscribePushCommand {
            user_id: stranger.id().clone(),
            endpoint: ENDPOINT.into(),
        },
        &deps,
    )
    .await
    .unwrap();

    assert_eq!(
        store.find_by_user(owner.id()).await.unwrap().len(),
        1,
        "the owner's device must still be subscribed"
    );
}

#[tokio::test]
async fn the_owner_can_unsubscribe_their_own_device() {
    let store = Arc::new(InMemoryStore::new());
    let owner = test_user("alice");

    let deps = subscribe::Deps {
        push_command: store.clone(),
        push_query: store.clone(),
    };
    subscribe::execute(a_subscription(owner.id().clone()), &deps)
        .await
        .unwrap();

    let deps = unsubscribe::Deps {
        push_command: store.clone(),
    };

    unsubscribe::execute(
        UnsubscribePushCommand {
            user_id: owner.id().clone(),
            endpoint: ENDPOINT.into(),
        },
        &deps,
    )
    .await
    .unwrap();

    assert!(store.find_by_user(owner.id()).await.unwrap().is_empty());
}
