use domain::errors::DomainError;
use domain::user::DisplayName;

#[test]
fn valid_display_name_is_created() {
    let name = DisplayName::new("Alice").unwrap();
    assert_eq!(name.value(), "Alice");
}

#[test]
fn display_name_trims_whitespace() {
    let name = DisplayName::new("  Bob  ").unwrap();
    assert_eq!(name.value(), "Bob");
}

#[test]
fn empty_display_name_is_rejected() {
    let result = DisplayName::new("");
    assert!(matches!(result, Err(DomainError::InvalidInput(_))));
}

#[test]
fn whitespace_only_display_name_is_rejected() {
    let result = DisplayName::new("   ");
    assert!(matches!(result, Err(DomainError::InvalidInput(_))));
}

#[test]
fn display_name_over_100_chars_is_rejected() {
    let long = "a".repeat(101);
    let result = DisplayName::new(long);
    assert!(matches!(result, Err(DomainError::InvalidInput(_))));
}

#[test]
fn display_name_at_100_chars_is_valid() {
    let exact = "a".repeat(100);
    let name = DisplayName::new(exact).unwrap();
    assert_eq!(name.value().len(), 100);
}
