use std::sync::Arc;

use domain::ports::ProviderConnectionQueryPort;
use domain::provider::ProviderName;
use domain::testing::{FakeCredentialCipher, InMemoryStore, test_user};

use application::provider::commands::ConnectProviderCommand;
use application::provider::use_cases::{connect_provider, disconnect_provider, list_connections};

const SUBSONIC_CREDENTIAL: &str =
    r#"{"url":"https://music.example","username":"gabriel","password":"hunter2"}"#;

fn deps() -> (Arc<InMemoryStore>, connect_provider::Deps) {
    let store = Arc::new(InMemoryStore::new());
    let deps = connect_provider::Deps {
        command: store.clone(),
        cipher: Arc::new(FakeCredentialCipher),
    };
    (store, deps)
}

#[tokio::test]
async fn connecting_stores_the_credential_sealed_rather_than_verbatim() {
    let (store, deps) = deps();
    let user = test_user("alice");

    let cmd = ConnectProviderCommand {
        user_id: user.id().clone(),
        provider: ProviderName::new("subsonic").unwrap(),
        credential: SUBSONIC_CREDENTIAL.as_bytes().to_vec(),
    };

    connect_provider::execute(cmd, &deps).await.unwrap();

    let stored = store.find_by_user(user.id()).await.unwrap();
    let sealed = stored[0].credential().value();

    assert!(
        !sealed.windows(7).any(|window| window == b"hunter2"),
        "the raw password reached storage"
    );
}

#[tokio::test]
async fn reconnecting_replaces_the_credential_rather_than_adding_a_second() {
    let (store, deps) = deps();
    let user = test_user("bob");
    let provider = ProviderName::new("subsonic").unwrap();

    for password in ["first", "second"] {
        let cmd = ConnectProviderCommand {
            user_id: user.id().clone(),
            provider: provider.clone(),
            credential: password.as_bytes().to_vec(),
        };
        connect_provider::execute(cmd, &deps).await.unwrap();
    }

    let stored = store.find_by_user(user.id()).await.unwrap();

    assert_eq!(stored.len(), 1);
}

#[tokio::test]
async fn listing_reports_connected_providers_without_their_credentials() {
    let (store, deps) = deps();
    let user = test_user("carol");

    connect_provider::execute(
        ConnectProviderCommand {
            user_id: user.id().clone(),
            provider: ProviderName::new("subsonic").unwrap(),
            credential: SUBSONIC_CREDENTIAL.as_bytes().to_vec(),
        },
        &deps,
    )
    .await
    .unwrap();

    let listing = list_connections::execute(
        user.id().clone(),
        &list_connections::Deps {
            query: store.clone(),
        },
    )
    .await
    .unwrap();

    assert_eq!(listing.len(), 1);
    assert_eq!(listing[0].provider.value(), "subsonic");
}

#[tokio::test]
async fn disconnecting_removes_the_connection() {
    let (store, deps) = deps();
    let user = test_user("dave");
    let provider = ProviderName::new("subsonic").unwrap();

    connect_provider::execute(
        ConnectProviderCommand {
            user_id: user.id().clone(),
            provider: provider.clone(),
            credential: SUBSONIC_CREDENTIAL.as_bytes().to_vec(),
        },
        &deps,
    )
    .await
    .unwrap();

    disconnect_provider::execute(
        user.id().clone(),
        provider,
        &disconnect_provider::Deps {
            command: store.clone(),
        },
    )
    .await
    .unwrap();

    assert!(store.find_by_user(user.id()).await.unwrap().is_empty());
}
