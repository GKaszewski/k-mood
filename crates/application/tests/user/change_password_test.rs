use std::sync::Arc;

use domain::ports::UserCommandPort;
use domain::testing::{FakePasswordHasher, InMemoryStore};
use domain::user::{Email, PasswordHash, User, Username};

use application::user::commands::ChangePasswordCommand;
use application::user::use_cases::change_password;

async fn setup() -> (Arc<InMemoryStore>, change_password::Deps, User) {
    let store = Arc::new(InMemoryStore::new());
    let user = User::new(
        Username::new("alice").unwrap(),
        Email::new("alice@example.com").unwrap(),
        PasswordHash::new("hashed:secret".into()),
    );
    store.save(&user).await.unwrap();

    let deps = change_password::Deps {
        user_command: store.clone(),
        user_query: store.clone(),
        password_hasher: Arc::new(FakePasswordHasher),
    };

    (store, deps, user)
}

#[tokio::test]
async fn changes_password_with_correct_current() {
    let (_store, deps, user) = setup().await;

    let cmd = ChangePasswordCommand {
        user_id: user.id().clone(),
        current_password: "secret".into(),
        new_password: "new-secret".into(),
    };

    change_password::execute(cmd, &deps).await.unwrap();
}

#[tokio::test]
async fn rejects_wrong_current_password() {
    let (_store, deps, user) = setup().await;

    let cmd = ChangePasswordCommand {
        user_id: user.id().clone(),
        current_password: "wrong".into(),
        new_password: "new-secret".into(),
    };

    let result = change_password::execute(cmd, &deps).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn rejects_nonexistent_user() {
    let (_store, deps, _user) = setup().await;

    let cmd = ChangePasswordCommand {
        user_id: domain::user::UserId::generate(),
        current_password: "secret".into(),
        new_password: "new-secret".into(),
    };

    let result = change_password::execute(cmd, &deps).await;
    assert!(result.is_err());
}
