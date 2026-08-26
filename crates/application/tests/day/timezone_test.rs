use std::sync::Arc;

use domain::ports::{UserCommandPort, UserQueryPort};
use domain::testing::{InMemoryStore, test_user};
use domain::user::Timezone;

use application::day::timezone_for;

#[tokio::test]
async fn a_users_timezone_is_resolved_when_they_have_one() {
    let store = Arc::new(InMemoryStore::new());
    let mut user = test_user("alice");
    user.update_timezone(Some(Timezone::new("Europe/Warsaw").unwrap()));
    store.save(&user).await.unwrap();

    let users: Arc<dyn UserQueryPort> = store.clone();
    let timezone = timezone_for(user.id(), &users).await.unwrap();

    assert_eq!(timezone.value(), "Europe/Warsaw");
}

#[tokio::test]
async fn a_user_with_no_timezone_gets_an_error_that_says_what_to_do() {
    let store = Arc::new(InMemoryStore::new());
    let user = test_user("bob");
    store.save(&user).await.unwrap();

    let users: Arc<dyn UserQueryPort> = store.clone();
    let error = timezone_for(user.id(), &users).await.unwrap_err();

    let message = error.to_string();
    assert!(message.contains("timezone"));
    assert!(message.contains("settings"));
}
