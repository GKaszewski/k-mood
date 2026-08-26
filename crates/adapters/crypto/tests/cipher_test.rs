use domain::provider::{CredentialCipher, EncryptedCredential};

use crypto::ChaChaCredentialCipher;

const TEST_KEY: &str = "gXqvVQF3v9pT2mKz8sYbN4jHcR7wLdE1uA0iO5tPxZk=";

fn cipher() -> ChaChaCredentialCipher {
    ChaChaCredentialCipher::new(TEST_KEY).unwrap()
}

#[test]
fn a_credential_survives_a_round_trip() {
    let cipher = cipher();
    let secret = br#"{"url":"https://music.example","username":"gabriel","password":"hunter2"}"#;

    let encrypted = cipher.encrypt(secret).unwrap();
    let decrypted = cipher.decrypt(&encrypted).unwrap();

    assert_eq!(decrypted, secret);
}

#[test]
fn the_plaintext_never_appears_in_the_ciphertext() {
    let cipher = cipher();

    let encrypted = cipher.encrypt(b"hunter2").unwrap();

    assert!(
        !encrypted
            .value()
            .windows(7)
            .any(|window| window == b"hunter2")
    );
}

#[test]
fn encrypting_the_same_secret_twice_gives_different_ciphertext() {
    let cipher = cipher();

    let first = cipher.encrypt(b"hunter2").unwrap();
    let second = cipher.encrypt(b"hunter2").unwrap();

    assert_ne!(first.value(), second.value());
}

#[test]
fn a_tampered_credential_is_rejected_rather_than_silently_decrypted() {
    let cipher = cipher();
    let encrypted = cipher.encrypt(b"hunter2").unwrap();

    let mut tampered = encrypted.value().to_vec();
    let last = tampered.len() - 1;
    tampered[last] ^= 0xff;

    let result = cipher.decrypt(&EncryptedCredential::from_persistence(tampered));

    assert!(result.is_err());
}

#[test]
fn a_credential_encrypted_under_another_key_is_rejected() {
    let other_key = "AAAAVQF3v9pT2mKz8sYbN4jHcR7wLdE1uA0iO5tPxZk=";
    let encrypted = cipher().encrypt(b"hunter2").unwrap();

    let result = ChaChaCredentialCipher::new(other_key)
        .unwrap()
        .decrypt(&encrypted);

    assert!(result.is_err());
}

#[test]
fn a_key_that_is_not_32_bytes_is_refused_at_construction() {
    assert!(ChaChaCredentialCipher::new("dGhpcyBpcyB0b28gc2hvcnQ=").is_err());
    assert!(ChaChaCredentialCipher::new("not-base64!!").is_err());
    assert!(ChaChaCredentialCipher::new("").is_err());
}
