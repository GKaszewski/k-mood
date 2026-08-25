use domain::activity::ActivityName;
use domain::errors::DomainError;

#[test]
fn valid_name_is_created() {
    let name = ActivityName::new("exercise").unwrap();
    assert_eq!(name.value(), "exercise");
}

#[test]
fn name_trims_whitespace() {
    let name = ActivityName::new("  friends  ").unwrap();
    assert_eq!(name.value(), "friends");
}

#[test]
fn empty_name_is_rejected() {
    let result = ActivityName::new("");
    assert!(matches!(result, Err(DomainError::InvalidInput(_))));
}

#[test]
fn whitespace_only_name_is_rejected() {
    let result = ActivityName::new("   ");
    assert!(matches!(result, Err(DomainError::InvalidInput(_))));
}
