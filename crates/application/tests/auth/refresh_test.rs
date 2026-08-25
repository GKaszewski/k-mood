use std::sync::Arc;

use chrono::{Duration, Utc};

use domain::auth::{RefreshSession, RefreshSessionId};
use domain::testing::InMemoryStore;
use domain::user::UserId;

use application::auth::use_cases::refresh;

fn store_with_session(user_id: UserId, ttl_seconds: i64) -> (Arc<InMemoryStore>, String) {
    let store = Arc::new(InMemoryStore::new());
    let session = RefreshSession::new(user_id, ttl_seconds);
    let token = session.token().to_string();
    store.refresh_sessions.write().unwrap().push(session);
    (store, token)
}

fn deps(store: &Arc<InMemoryStore>) -> refresh::Deps {
    refresh::Deps {
        auth_service: store.clone(),
        refresh_session_command: store.clone(),
        refresh_session_query: store.clone(),
        refresh_token_ttl_seconds: 86400,
    }
}

#[tokio::test]
async fn rotates_refresh_token() {
    let user_id = UserId::generate();
    let (store, token) = store_with_session(user_id, 86400);

    let result = refresh::execute(&token, &deps(&store)).await.unwrap();

    assert!(!result.access_token.token().is_empty());
    assert_ne!(result.refresh_token, token);
}

#[tokio::test]
async fn rejects_unknown_token() {
    let store = Arc::new(InMemoryStore::new());
    let result = refresh::execute("nonexistent-token", &deps(&store)).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn rejects_expired_token() {
    let store = Arc::new(InMemoryStore::new());
    let user_id = UserId::generate();
    let session = RefreshSession::from_persistence(
        RefreshSessionId::generate(),
        user_id,
        "expired-token".into(),
        Utc::now() - Duration::seconds(1),
        Utc::now() - Duration::hours(1),
    );
    store.refresh_sessions.write().unwrap().push(session);

    let result = refresh::execute("expired-token", &deps(&store)).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn old_token_is_revoked_after_rotation() {
    let user_id = UserId::generate();
    let (store, token) = store_with_session(user_id, 86400);

    refresh::execute(&token, &deps(&store)).await.unwrap();

    let old_session = store
        .refresh_sessions
        .read()
        .unwrap()
        .iter()
        .find(|s| s.token() == token)
        .cloned();
    assert!(old_session.is_none());
}
