use domain::attachment::ContentType;
use domain::errors::DomainError;

#[test]
fn valid_content_type_is_created() {
    let ct = ContentType::new("image/jpeg").unwrap();
    assert_eq!(ct.value(), "image/jpeg");
}

#[test]
fn content_type_is_lowercased() {
    let ct = ContentType::new("Image/JPEG").unwrap();
    assert_eq!(ct.value(), "image/jpeg");
}

#[test]
fn content_type_trims_whitespace() {
    let ct = ContentType::new("  audio/webm  ").unwrap();
    assert_eq!(ct.value(), "audio/webm");
}

#[test]
fn empty_content_type_is_rejected() {
    let result = ContentType::new("");
    assert!(matches!(result, Err(DomainError::InvalidInput(_))));
}

#[test]
fn content_type_without_slash_is_rejected() {
    let result = ContentType::new("jpeg");
    assert!(matches!(result, Err(DomainError::InvalidInput(_))));
}

#[test]
fn from_persistence_bypasses_validation() {
    let ct = ContentType::from_persistence(String::new());
    assert_eq!(ct.value(), "");
}
