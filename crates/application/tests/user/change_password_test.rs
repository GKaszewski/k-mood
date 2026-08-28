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
        refresh_sessions: store.clone(),
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

#[tokio::test]
async fn changing_a_password_revokes_every_live_session() {
    let (store, deps, user) = setup().await;

    for _ in 0..3 {
        domain::ports::RefreshSessionCommandPort::create(
            store.as_ref(),
            &domain::auth::RefreshSession::new(user.id().clone(), 3_600),
        )
        .await
        .unwrap();
    }

    assert_eq!(store.refresh_sessions.read().unwrap().len(), 3);

    change_password::execute(
        ChangePasswordCommand {
            user_id: user.id().clone(),
            current_password: "secret".into(),
            new_password: "new-secret".into(),
        },
        &deps,
    )
    .await
    .unwrap();

    assert!(
        store.refresh_sessions.read().unwrap().is_empty(),
        "a stolen refresh token must not survive a password change"
    );
}

#[tokio::test]
async fn a_refused_password_change_leaves_sessions_alone() {
    let (store, deps, user) = setup().await;

    domain::ports::RefreshSessionCommandPort::create(
        store.as_ref(),
        &domain::auth::RefreshSession::new(user.id().clone(), 3_600),
    )
    .await
    .unwrap();

    let refused = change_password::execute(
        ChangePasswordCommand {
            user_id: user.id().clone(),
            current_password: "not-the-password".into(),
            new_password: "new-secret".into(),
        },
        &deps,
    )
    .await;

    assert!(refused.is_err());
    assert_eq!(
        store.refresh_sessions.read().unwrap().len(),
        1,
        "a failed attempt must not log the account out"
    );
}
