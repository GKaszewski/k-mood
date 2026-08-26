use domain::ports::ApiTokenSecretPort;

use auth::ApiTokenSecret;

const HEX_CHARACTERS_IN_A_SHA256: usize = 64;

#[test]
fn a_minted_secret_announces_itself_so_a_scanner_can_spot_it() {
    let minted = ApiTokenSecret.mint();

    assert!(minted.starts_with("kmood_"), "got {minted}");
    assert!(minted.len() > 40, "a secret needs real entropy: {minted}");
}

#[test]
fn no_two_secrets_are_alike() {
    let secrets: Vec<String> = (0..50).map(|_| ApiTokenSecret.mint()).collect();
    let mut unique = secrets.clone();
    unique.sort();
    unique.dedup();

    assert_eq!(unique.len(), secrets.len());
}

#[test]
fn the_same_secret_always_digests_the_same_way() {
    let secret = ApiTokenSecret.mint();

    assert_eq!(
        ApiTokenSecret.digest(&secret).value(),
        ApiTokenSecret.digest(&secret).value()
    );
}

#[test]
fn different_secrets_digest_differently() {
    let first = ApiTokenSecret.digest("kmood_one");
    let second = ApiTokenSecret.digest("kmood_two");

    assert_ne!(first.value(), second.value());
}

#[test]
fn a_digest_gives_nothing_of_the_secret_away() {
    let secret = ApiTokenSecret.mint();
    let digest = ApiTokenSecret.digest(&secret);

    assert_eq!(digest.value().len(), HEX_CHARACTERS_IN_A_SHA256);
    assert!(!digest.value().contains(&secret));
    assert!(!secret.contains(digest.value()));
    assert!(digest.value().chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn the_digest_matches_the_published_value_for_a_known_input() {
    let digest = ApiTokenSecret.digest("abc");

    assert_eq!(
        digest.value(),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}
