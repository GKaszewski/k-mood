use std::sync::Arc;

use domain::api_token::ApiToken;
use domain::ports::UserCommandPort;
use domain::provider::ProviderName;
use domain::testing::{FakeApiTokenSecret, InMemoryStore, test_user};
use domain::user::{User, UserId};

use application::api_token::commands::MintApiTokenCommand;
use application::api_token::use_cases::{
    authenticate_api_token, list_api_tokens, mint_api_token, revoke_api_token,
};

struct Fixture {
    store: Arc<InMemoryStore>,
    secrets: Arc<FakeApiTokenSecret>,
    user: User,
}

async fn a_user_with_no_tokens() -> Fixture {
    let store = Arc::new(InMemoryStore::new());
    let user = test_user("alice");
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();

    Fixture {
        store,
        secrets: Arc::new(FakeApiTokenSecret::new()),
        user,
    }
}

impl Fixture {
    async fn mint(&self, name: &str) -> Result<String, application::errors::ApplicationError> {
        let deps = mint_api_token::Deps {
            command: self.store.clone(),
            secrets: self.secrets.clone(),
        };

        let minted = mint_api_token::execute(
            MintApiTokenCommand {
                user_id: self.user.id().clone(),
                name: ProviderName::new(name)?,
            },
            &deps,
        )
        .await?;

        Ok(minted.secret().to_string())
    }

    async fn authenticate(&self, secret: &str) -> Option<ApiToken> {
        let deps = authenticate_api_token::Deps {
            query: self.store.clone(),
            command: self.store.clone(),
            secrets: self.secrets.clone(),
        };

        authenticate_api_token::execute(secret, &deps).await.ok()
    }

    async fn list(&self) -> Vec<ApiToken> {
        let deps = list_api_tokens::Deps {
            query: self.store.clone(),
        };

        list_api_tokens::execute(self.user.id().clone(), &deps)
            .await
            .unwrap()
    }

    async fn revoke(
        &self,
        owner: UserId,
        token: &ApiToken,
    ) -> Result<(), application::errors::ApplicationError> {
        let deps = revoke_api_token::Deps {
            command: self.store.clone(),
        };

        revoke_api_token::execute(owner, token.id().clone(), &deps).await
    }
}

#[tokio::test]
async fn a_minted_token_is_returned_once_and_stored_only_as_a_digest() {
    let fixture = a_user_with_no_tokens().await;

    let secret = fixture.mint("iphone-shortcuts").await.unwrap();

    let stored = fixture.list().await;
    assert_eq!(stored.len(), 1);
    assert_eq!(stored[0].name().value(), "iphone-shortcuts");
    assert!(!secret.is_empty(), "the caller is handed the secret once");
    assert_ne!(
        stored[0].digest().value(),
        secret,
        "what is stored is a digest, not the secret itself"
    );
}

#[tokio::test]
async fn the_secret_authenticates_and_names_the_provider_its_writes_belong_to() {
    let fixture = a_user_with_no_tokens().await;
    let secret = fixture.mint("iphone-shortcuts").await.unwrap();

    let token = fixture.authenticate(&secret).await.expect("it should work");

    assert_eq!(token.user_id(), fixture.user.id());
    assert_eq!(token.name().value(), "iphone-shortcuts");
}

#[tokio::test]
async fn a_secret_nobody_minted_authenticates_nothing() {
    let fixture = a_user_with_no_tokens().await;
    fixture.mint("iphone-shortcuts").await.unwrap();

    assert!(
        fixture
            .authenticate("kmood_not_a_real_secret")
            .await
            .is_none()
    );
}

#[tokio::test]
async fn a_revoked_token_stops_working_at_once() {
    let fixture = a_user_with_no_tokens().await;
    let secret = fixture.mint("iphone-shortcuts").await.unwrap();
    let token = fixture.authenticate(&secret).await.unwrap();

    fixture
        .revoke(fixture.user.id().clone(), &token)
        .await
        .unwrap();

    assert!(fixture.authenticate(&secret).await.is_none());
    assert!(fixture.list().await.is_empty());
}

#[tokio::test]
async fn using_a_token_records_that_it_was_used() {
    let fixture = a_user_with_no_tokens().await;
    let secret = fixture.mint("iphone-shortcuts").await.unwrap();

    assert!(fixture.list().await[0].last_used_at().is_none());

    fixture.authenticate(&secret).await.unwrap();

    assert!(fixture.list().await[0].last_used_at().is_some());
}

#[tokio::test]
async fn nobody_can_revoke_a_token_that_is_not_theirs() {
    let fixture = a_user_with_no_tokens().await;
    let secret = fixture.mint("iphone-shortcuts").await.unwrap();
    let token = fixture.authenticate(&secret).await.unwrap();

    let attempt = fixture.revoke(UserId::generate(), &token).await;

    assert!(attempt.is_err());
    assert!(fixture.authenticate(&secret).await.is_some());
}

#[tokio::test]
async fn two_tokens_cannot_share_a_name_because_the_name_is_the_provider() {
    let fixture = a_user_with_no_tokens().await;
    fixture.mint("iphone-shortcuts").await.unwrap();

    let again = fixture.mint("iphone-shortcuts").await;

    assert!(again.is_err());
    assert_eq!(fixture.list().await.len(), 1);
}

#[tokio::test]
async fn every_minting_produces_a_different_secret() {
    let fixture = a_user_with_no_tokens().await;

    let first = fixture.mint("iphone-shortcuts").await.unwrap();
    let second = fixture.mint("tasker").await.unwrap();

    assert_ne!(first, second);
}
