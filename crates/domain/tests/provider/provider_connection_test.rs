use domain::provider::{EncryptedCredential, ProviderConnection, ProviderName};
use domain::user::UserId;

fn credential() -> EncryptedCredential {
    EncryptedCredential::from_persistence(vec![1, 2, 3, 4])
}

#[test]
fn a_provider_name_is_a_lowercase_slug() {
    assert_eq!(ProviderName::new("Subsonic").unwrap().value(), "subsonic");
    assert_eq!(ProviderName::new("  spotify ").unwrap().value(), "spotify");
}

#[test]
fn a_blank_or_malformed_provider_name_is_rejected() {
    assert!(ProviderName::new("").is_err());
    assert!(ProviderName::new("   ").is_err());
    assert!(ProviderName::new("my provider").is_err());
}

#[test]
fn a_connection_belongs_to_one_user_and_one_provider() {
    let user_id = UserId::generate();
    let provider = ProviderName::new("subsonic").unwrap();

    let connection = ProviderConnection::new(user_id.clone(), provider.clone(), credential());

    assert_eq!(*connection.user_id(), user_id);
    assert_eq!(connection.provider().value(), "subsonic");
}

#[test]
fn a_credential_never_reveals_itself_in_debug_output() {
    let user_id = UserId::generate();
    let provider = ProviderName::new("subsonic").unwrap();
    let secret = EncryptedCredential::from_persistence(b"hunter2".to_vec());

    let connection = ProviderConnection::new(user_id, provider, secret);

    let rendered = format!("{connection:?}");
    assert!(!rendered.contains("hunter2"));
    assert!(!rendered.contains("104, 117, 110"));
}

#[test]
fn replacing_a_credential_touches_the_connection() {
    let user_id = UserId::generate();
    let provider = ProviderName::new("subsonic").unwrap();
    let mut connection = ProviderConnection::new(user_id, provider, credential());
    let before = *connection.updated_at();

    connection.replace_credential(EncryptedCredential::from_persistence(vec![9, 9]));

    assert!(*connection.updated_at() >= before);
}
