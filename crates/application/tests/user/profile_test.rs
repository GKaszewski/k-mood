use std::sync::Arc;

use domain::ports::UserCommandPort;
use domain::testing::{InMemoryStore, test_user};
use domain::user::{DisplayName, Timezone};

use application::user::commands::UpdateProfileCommand;
use application::user::use_cases::{get_profile, update_profile};

#[tokio::test]
async fn get_profile_returns_user() {
    let store = Arc::new(InMemoryStore::new());
    let user = test_user("alice");
    store.save(&user).await.unwrap();

    let deps = get_profile::Deps {
        user_query: store.clone(),
    };

    let result = get_profile::execute(user.id().clone(), &deps)
        .await
        .unwrap();
    assert_eq!(result.username().value(), "alice");
}

#[tokio::test]
async fn updates_display_name_and_timezone() {
    let store = Arc::new(InMemoryStore::new());
    let user = test_user("alice");
    store.save(&user).await.unwrap();

    let deps = update_profile::Deps {
        user_command: store.clone(),
        user_query: store.clone(),
    };

    let cmd = UpdateProfileCommand {
        user_id: user.id().clone(),
        display_name: Some(DisplayName::new("Alice K").unwrap()),
        timezone: Some(Timezone::new("Europe/Warsaw").unwrap()),
    };

    let updated = update_profile::execute(cmd, &deps).await.unwrap();

    assert_eq!(updated.display_name().map(|d| d.value()), Some("Alice K"));
    assert_eq!(updated.timezone().map(|t| t.value()), Some("Europe/Warsaw"));
}
