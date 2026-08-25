use domain::errors::DomainError;
use domain::user::Username;

#[test]
fn valid_username_is_created() {
    let username = Username::new("gabriel").unwrap();
    assert_eq!(username.value(), "gabriel");
}

#[test]
fn username_with_dots_and_underscores_is_valid() {
    let username = Username::new("gabriel.k_99").unwrap();
    assert_eq!(username.value(), "gabriel.k_99");
}

#[test]
fn username_trims_whitespace() {
    let username = Username::new("  gabriel  ").unwrap();
    assert_eq!(username.value(), "gabriel");
}

#[test]
fn empty_username_is_rejected() {
    let result = Username::new("");
    assert!(matches!(result, Err(DomainError::InvalidInput(_))));
}

#[test]
fn username_longer_than_32_chars_is_rejected() {
    let long = "a".repeat(33);
    let result = Username::new(long);
    assert!(matches!(result, Err(DomainError::InvalidInput(_))));
}

#[test]
fn username_with_special_chars_is_rejected() {
    let result = Username::new("gabriel@home");
    assert!(matches!(result, Err(DomainError::InvalidInput(_))));
}

#[test]
fn username_with_spaces_is_rejected() {
    let result = Username::new("gabriel k");
    assert!(matches!(result, Err(DomainError::InvalidInput(_))));
}
