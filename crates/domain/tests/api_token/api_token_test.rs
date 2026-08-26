use domain::api_token::{ApiToken, MintedApiToken, TokenDigest, TokenScope};
use domain::provider::ProviderName;
use domain::user::UserId;

fn a_token() -> ApiToken {
    ApiToken::new(
        UserId::generate(),
        ProviderName::new("iphone-shortcuts").unwrap(),
        TokenDigest::from_persistence("digest-of-the-secret".into()),
    )
}

#[test]
fn a_fresh_token_has_never_been_used() {
    let token = a_token();

    assert!(token.last_used_at().is_none());
    assert_eq!(token.scope(), TokenScope::WriteMetrics);
}

#[test]
fn a_token_remembers_when_it_was_last_used() {
    let mut token = a_token();

    token.mark_used();

    assert!(token.last_used_at().is_some());
}

#[test]
fn the_name_a_token_carries_is_the_provider_its_writes_are_attributed_to() {
    let token = a_token();

    assert_eq!(token.name().value(), "iphone-shortcuts");
}

#[test]
fn every_scope_survives_a_round_trip_through_its_name() {
    assert_eq!(
        TokenScope::from_name(TokenScope::WriteMetrics.name()),
        Some(TokenScope::WriteMetrics)
    );
    assert_eq!(TokenScope::from_name("read-everything"), None);
}

#[test]
fn a_minted_token_keeps_the_secret_out_of_its_debug_output() {
    let minted = MintedApiToken::new(a_token(), "kmood_supersecretvalue".into());

    let printed = format!("{minted:?}");

    assert!(
        !printed.contains("supersecret"),
        "the secret leaked: {printed}"
    );
    assert_eq!(minted.secret(), "kmood_supersecretvalue");
}
