use sqlx::sqlite::SqlitePoolOptions;

use domain::api_token::{ApiToken, TokenDigest};
use domain::ports::{ApiTokenCommandPort, ApiTokenQueryPort, CascadeDeletePort, UserCommandPort};
use domain::provider::ProviderName;
use domain::testing::test_user;
use domain::user::{User, UserId};

use sqlite::repositories::{
    SqliteApiTokenCommandRepository, SqliteApiTokenQueryRepository, SqliteCascadeDeleteRepository,
    SqliteUserCommandRepository,
};

async fn a_pool_with_a_user() -> (sqlx::SqlitePool, User) {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlite::run_migrations(&pool).await.unwrap();

    let user = test_user("alice");
    SqliteUserCommandRepository::new(pool.clone())
        .save(&user)
        .await
        .unwrap();

    (pool, user)
}

fn a_token(owner: &UserId, name: &str, digest: &str) -> ApiToken {
    ApiToken::new(
        owner.clone(),
        ProviderName::new(name).unwrap(),
        TokenDigest::from_persistence(digest.into()),
    )
}

#[tokio::test]
async fn a_token_is_found_by_the_digest_of_its_secret() {
    let (pool, user) = a_pool_with_a_user().await;
    let token = a_token(user.id(), "iphone-shortcuts", "abc123");

    SqliteApiTokenCommandRepository::new(pool.clone())
        .save(&token)
        .await
        .unwrap();

    let found = SqliteApiTokenQueryRepository::new(pool.clone())
        .find_by_digest(&TokenDigest::from_persistence("abc123".into()))
        .await
        .unwrap()
        .expect("the token should be found");

    assert_eq!(found.id(), token.id());
    assert_eq!(found.name().value(), "iphone-shortcuts");
    assert!(found.last_used_at().is_none());
}

#[tokio::test]
async fn a_digest_nobody_stored_finds_nothing() {
    let (pool, _) = a_pool_with_a_user().await;

    let found = SqliteApiTokenQueryRepository::new(pool.clone())
        .find_by_digest(&TokenDigest::from_persistence("nothing".into()))
        .await
        .unwrap();

    assert!(found.is_none());
}

#[tokio::test]
async fn two_tokens_of_one_account_cannot_share_a_name() {
    let (pool, user) = a_pool_with_a_user().await;
    let tokens = SqliteApiTokenCommandRepository::new(pool.clone());

    tokens
        .save(&a_token(user.id(), "iphone-shortcuts", "first"))
        .await
        .unwrap();

    let again = tokens
        .save(&a_token(user.id(), "iphone-shortcuts", "second"))
        .await;

    let refusal = again
        .expect_err("a duplicate name must be refused")
        .to_string();

    assert!(
        refusal.contains("already exists"),
        "the refusal should say what is wrong, got: {refusal}"
    );
}

#[tokio::test]
async fn using_a_token_is_recorded_against_it() {
    let (pool, user) = a_pool_with_a_user().await;
    let token = a_token(user.id(), "tasker", "abc123");
    let tokens = SqliteApiTokenCommandRepository::new(pool.clone());
    tokens.save(&token).await.unwrap();

    tokens.mark_used(token.id()).await.unwrap();

    let found = SqliteApiTokenQueryRepository::new(pool.clone())
        .find_by_digest(&TokenDigest::from_persistence("abc123".into()))
        .await
        .unwrap()
        .unwrap();

    assert!(found.last_used_at().is_some());
}

#[tokio::test]
async fn revoking_a_token_removes_it_for_good() {
    let (pool, user) = a_pool_with_a_user().await;
    let token = a_token(user.id(), "tasker", "abc123");
    let tokens = SqliteApiTokenCommandRepository::new(pool.clone());
    tokens.save(&token).await.unwrap();

    tokens.revoke(user.id(), token.id()).await.unwrap();

    let found = SqliteApiTokenQueryRepository::new(pool.clone())
        .find_by_digest(&TokenDigest::from_persistence("abc123".into()))
        .await
        .unwrap();

    assert!(found.is_none());
}

#[tokio::test]
async fn a_token_belonging_to_someone_else_cannot_be_revoked() {
    let (pool, user) = a_pool_with_a_user().await;
    let token = a_token(user.id(), "tasker", "abc123");
    let tokens = SqliteApiTokenCommandRepository::new(pool.clone());
    tokens.save(&token).await.unwrap();

    let attempt = tokens.revoke(&UserId::generate(), token.id()).await;

    assert!(attempt.is_err());
    assert!(
        SqliteApiTokenQueryRepository::new(pool.clone())
            .find_by_digest(&TokenDigest::from_persistence("abc123".into()))
            .await
            .unwrap()
            .is_some()
    );
}

#[tokio::test]
async fn a_row_with_a_scope_this_build_does_not_know_authenticates_nothing() {
    let (pool, user) = a_pool_with_a_user().await;

    sqlx::query(
        "INSERT INTO api_tokens (id, user_id, name, digest, scope, created_at, last_used_at)
         VALUES (?, ?, ?, ?, ?, ?, NULL)",
    )
    .bind(uuid::Uuid::new_v4().to_string())
    .bind(user.id().value().to_string())
    .bind("legacy")
    .bind("abc123")
    .bind("readEverything")
    .bind(chrono::Utc::now().to_rfc3339())
    .execute(&pool)
    .await
    .unwrap();

    let found = SqliteApiTokenQueryRepository::new(pool.clone())
        .find_by_digest(&TokenDigest::from_persistence("abc123".into()))
        .await
        .unwrap();

    assert!(
        found.is_none(),
        "an unreadable scope must not grant anything"
    );
}

#[tokio::test]
async fn deleting_an_account_takes_its_tokens_with_it() {
    let (pool, user) = a_pool_with_a_user().await;
    SqliteApiTokenCommandRepository::new(pool.clone())
        .save(&a_token(user.id(), "tasker", "abc123"))
        .await
        .unwrap();

    SqliteCascadeDeleteRepository::new(pool.clone())
        .delete_user_account(user.id())
        .await
        .unwrap();

    let remaining: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM api_tokens")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(remaining.0, 0);
}
