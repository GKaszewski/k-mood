use std::sync::Arc;

use domain::ports::UserCommandPort;
use domain::testing::{FakePasswordHasher, InMemoryStore};
use domain::user::{Email, PasswordHash, User, Username};

use application::auth::use_cases::login;
use application::user::commands::LoginCommand;

async fn setup() -> (Arc<InMemoryStore>, login::Deps) {
    let store = Arc::new(InMemoryStore::new());
    let user = User::new(
        Username::new("alice").unwrap(),
        Email::new("alice@example.com").unwrap(),
        PasswordHash::new("hashed:secret".into()),
    );
    store.save(&user).await.unwrap();

    let deps = login::Deps {
        user_query: store.clone(),
        password_hasher: Arc::new(FakePasswordHasher),
        auth_service: store.clone(),
        refresh_session_command: store.clone(),
        refresh_token_ttl_seconds: 86400,
    };

    (store, deps)
}

#[tokio::test]
async fn login_with_username() {
    let (_, deps) = setup().await;

    let cmd = LoginCommand {
        identifier: "alice".into(),
        password: "secret".into(),
    };

    let result = login::execute(cmd, &deps).await.unwrap();
    assert_eq!(result.user.username().value(), "alice");
    assert!(!result.access_token.token().is_empty());
    assert!(!result.refresh_token.is_empty());
}

#[tokio::test]
async fn login_with_email() {
    let (_, deps) = setup().await;

    let cmd = LoginCommand {
        identifier: "alice@example.com".into(),
        password: "secret".into(),
    };

    let result = login::execute(cmd, &deps).await.unwrap();
    assert_eq!(result.user.username().value(), "alice");
}

#[tokio::test]
async fn wrong_password_fails() {
    let (_, deps) = setup().await;

    let cmd = LoginCommand {
        identifier: "alice".into(),
        password: "wrong".into(),
    };

    let result = login::execute(cmd, &deps).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn nonexistent_user_fails() {
    let (_, deps) = setup().await;

    let cmd = LoginCommand {
        identifier: "nobody".into(),
        password: "secret".into(),
    };

    let result = login::execute(cmd, &deps).await;
    assert!(result.is_err());
}
