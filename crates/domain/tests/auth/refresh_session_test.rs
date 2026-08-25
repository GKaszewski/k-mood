use chrono::{Duration, Utc};

use domain::auth::{RefreshSession, RefreshSessionId};
use domain::user::UserId;

#[test]
fn new_session_is_not_expired() {
    let session = RefreshSession::new(UserId::generate(), 3600);
    assert!(!session.is_expired());
}

#[test]
fn session_with_past_expiry_is_expired() {
    let session = RefreshSession::from_persistence(
        RefreshSessionId::generate(),
        UserId::generate(),
        "token".into(),
        Utc::now() - Duration::seconds(1),
        Utc::now() - Duration::seconds(3600),
    );
    assert!(session.is_expired());
}

#[test]
fn new_session_has_unique_token() {
    let user_id = UserId::generate();
    let a = RefreshSession::new(user_id.clone(), 3600);
    let b = RefreshSession::new(user_id, 3600);
    assert_ne!(a.token(), b.token());
}

#[test]
fn session_stores_user_id() {
    let user_id = UserId::generate();
    let session = RefreshSession::new(user_id.clone(), 3600);
    assert_eq!(*session.user_id(), user_id);
}
