use domain::errors::DomainError;
use domain::user::Timezone;

#[test]
fn valid_timezone_is_created() {
    let tz = Timezone::new("Europe/Warsaw").unwrap();
    assert_eq!(tz.value(), "Europe/Warsaw");
}

#[test]
fn timezone_trims_whitespace() {
    let tz = Timezone::new("  America/New_York  ").unwrap();
    assert_eq!(tz.value(), "America/New_York");
}

#[test]
fn empty_timezone_is_rejected() {
    let result = Timezone::new("");
    assert!(matches!(result, Err(DomainError::InvalidInput(_))));
}

#[test]
fn timezone_without_slash_is_rejected() {
    let result = Timezone::new("UTC");
    assert!(matches!(result, Err(DomainError::InvalidInput(_))));
}

#[test]
fn from_persistence_bypasses_validation() {
    let tz = Timezone::from_persistence("UTC".into());
    assert_eq!(tz.value(), "UTC");
}
