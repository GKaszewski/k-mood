use domain::errors::DomainError;
use domain::user::Email;

#[test]
fn valid_email_is_created() {
    let email = Email::new("user@example.com").unwrap();
    assert_eq!(email.value(), "user@example.com");
}

#[test]
fn email_is_lowercased() {
    let email = Email::new("User@Example.COM").unwrap();
    assert_eq!(email.value(), "user@example.com");
}

#[test]
fn email_trims_whitespace() {
    let email = Email::new("  user@example.com  ").unwrap();
    assert_eq!(email.value(), "user@example.com");
}

#[test]
fn email_without_at_is_rejected() {
    let result = Email::new("not-an-email");
    assert!(matches!(result, Err(DomainError::InvalidInput(_))));
}

#[test]
fn empty_email_is_rejected() {
    let result = Email::new("");
    assert!(matches!(result, Err(DomainError::InvalidInput(_))));
}
