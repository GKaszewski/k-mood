use domain::entry::Content;
use domain::errors::DomainError;

#[test]
fn valid_content_is_created() {
    let content = Content::new("went for a walk").unwrap();
    assert_eq!(content.value(), "went for a walk");
}

#[test]
fn content_trims_whitespace() {
    let content = Content::new("  hello world  ").unwrap();
    assert_eq!(content.value(), "hello world");
}

#[test]
fn empty_content_is_rejected() {
    let result = Content::new("");
    assert!(matches!(result, Err(DomainError::InvalidInput(_))));
}

#[test]
fn whitespace_only_content_is_rejected() {
    let result = Content::new("   ");
    assert!(matches!(result, Err(DomainError::InvalidInput(_))));
}

#[test]
fn from_persistence_bypasses_validation() {
    let content = Content::from_persistence(String::new());
    assert_eq!(content.value(), "");
}
