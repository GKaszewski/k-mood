use domain::attachment::{ContentType, MediaUpload};
use domain::errors::DomainError;

#[test]
fn valid_upload_is_created() {
    let ct = ContentType::new("image/png").unwrap();
    let upload = MediaUpload::new(vec![1, 2, 3], ct).unwrap();

    assert_eq!(upload.data(), &[1, 2, 3]);
    assert_eq!(upload.content_type().value(), "image/png");
    assert_eq!(upload.size(), 3);
}

#[test]
fn empty_data_is_rejected() {
    let ct = ContentType::new("image/png").unwrap();
    let result = MediaUpload::new(vec![], ct);
    assert!(matches!(result, Err(DomainError::InvalidInput(_))));
}
