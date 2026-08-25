use domain::activity::CategoryName;
use domain::errors::DomainError;

#[test]
fn valid_category_name_is_created() {
    let name = CategoryName::new("hobbies").unwrap();
    assert_eq!(name.value(), "hobbies");
}

#[test]
fn category_name_trims_whitespace() {
    let name = CategoryName::new("  social  ").unwrap();
    assert_eq!(name.value(), "social");
}

#[test]
fn empty_category_name_is_rejected() {
    let result = CategoryName::new("");
    assert!(matches!(result, Err(DomainError::InvalidInput(_))));
}

#[test]
fn whitespace_only_category_name_is_rejected() {
    let result = CategoryName::new("   ");
    assert!(matches!(result, Err(DomainError::InvalidInput(_))));
}

#[test]
fn from_persistence_bypasses_validation() {
    let name = CategoryName::from_persistence(String::new());
    assert_eq!(name.value(), "");
}
