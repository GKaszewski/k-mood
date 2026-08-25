use std::sync::Arc;

use domain::testing::{FakePasswordHasher, InMemoryStore};
use domain::user::Username;

use application::user::commands::RegisterCommand;
use application::user::use_cases::register;

fn deps() -> (Arc<InMemoryStore>, register::Deps) {
    let store = Arc::new(InMemoryStore::new());
    let deps = register::Deps {
        user_command: store.clone(),
        user_query: store.clone(),
        activity_command: store.clone(),
        password_hasher: Arc::new(FakePasswordHasher),
        events: store.clone(),
        preset: config::PresetConfig::default(),
    };
    (store, deps)
}

#[tokio::test]
async fn registers_user_and_seeds_activities() {
    let (store, deps) = deps();

    let cmd = RegisterCommand {
        username: Username::new("alice").unwrap(),
        email: domain::user::Email::new("alice@example.com").unwrap(),
        password: "secret123".into(),
    };

    let user = register::execute(cmd, &deps).await.unwrap();

    assert_eq!(user.username().value(), "alice");
    assert!(store.activity_count() > 0);
    assert_eq!(store.published_events().len(), 1);
}

#[tokio::test]
async fn rejects_duplicate_username() {
    let (_, deps) = deps();

    let cmd1 = RegisterCommand {
        username: Username::new("alice").unwrap(),
        email: domain::user::Email::new("alice@example.com").unwrap(),
        password: "secret".into(),
    };
    register::execute(cmd1, &deps).await.unwrap();

    let cmd2 = RegisterCommand {
        username: Username::new("alice").unwrap(),
        email: domain::user::Email::new("alice2@example.com").unwrap(),
        password: "secret".into(),
    };
    let result = register::execute(cmd2, &deps).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn rejects_duplicate_email() {
    let (_, deps) = deps();

    let cmd1 = RegisterCommand {
        username: Username::new("alice").unwrap(),
        email: domain::user::Email::new("alice@example.com").unwrap(),
        password: "secret".into(),
    };
    register::execute(cmd1, &deps).await.unwrap();

    let cmd2 = RegisterCommand {
        username: Username::new("bob").unwrap(),
        email: domain::user::Email::new("alice@example.com").unwrap(),
        password: "secret".into(),
    };
    let result = register::execute(cmd2, &deps).await;

    assert!(result.is_err());
}
