use domain::api_token::{ApiToken, MintedApiToken, TokenDigest, TokenScope, TokenScopes};
use domain::provider::ProviderName;
use domain::user::UserId;

fn a_token() -> ApiToken {
    token_granting([TokenScope::WriteMetrics])
}

fn token_granting(scopes: impl IntoIterator<Item = TokenScope>) -> ApiToken {
    ApiToken::new(
        UserId::generate(),
        ProviderName::new("iphone-shortcuts").unwrap(),
        TokenDigest::from_persistence("digest-of-the-secret".into()),
        TokenScopes::new(scopes).unwrap(),
    )
}

#[test]
fn a_fresh_token_has_never_been_used() {
    let token = a_token();

    assert!(token.last_used_at().is_none());
    assert!(token.allows(TokenScope::WriteMetrics));
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
    for scope in TokenScope::ALL {
        assert_eq!(TokenScope::from_name(scope.name()), Some(scope));
    }

    assert_eq!(TokenScope::from_name("read-everything"), None);
}

#[test]
fn a_token_grants_only_what_it_was_minted_for() {
    let token = token_granting([TokenScope::ReadJournal]);

    assert!(token.allows(TokenScope::ReadJournal));
    assert!(!token.allows(TokenScope::WriteJournal));
    assert!(!token.allows(TokenScope::WriteMetrics));
    assert!(!token.allows(TokenScope::ReadProfile));
}

#[test]
fn a_token_that_grants_nothing_cannot_be_minted() {
    assert!(TokenScopes::new([]).is_err());
    assert!(TokenScopes::from_names(Vec::<&str>::new()).is_err());
}

#[test]
fn an_unknown_scope_name_is_refused_rather_than_ignored() {
    let refused = TokenScopes::from_names(["readJournal", "readEverything"]);

    assert!(
        refused.is_err(),
        "a typo must not quietly mint a narrower token than asked for"
    );
}

#[test]
fn scopes_survive_a_round_trip_through_persistence() {
    let held = TokenScopes::new([TokenScope::ReadJournal, TokenScope::WriteJournal]).unwrap();

    assert_eq!(
        TokenScopes::from_persistence(&held.to_persistence()),
        Some(held)
    );
}

#[test]
fn a_stored_scope_this_build_cannot_read_makes_the_whole_set_unreadable() {
    assert_eq!(
        TokenScopes::from_persistence("readJournal writeEverything"),
        None
    );
    assert_eq!(TokenScopes::from_persistence(""), None);
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
