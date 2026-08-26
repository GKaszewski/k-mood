use domain::errors::DomainError;
use domain::user::Timezone;

#[test]
fn a_zone_from_the_iana_database_is_accepted() {
    assert_eq!(
        Timezone::new("Europe/Warsaw").unwrap().value(),
        "Europe/Warsaw"
    );
    assert_eq!(
        Timezone::new("America/New_York").unwrap().value(),
        "America/New_York"
    );
}

#[test]
fn utc_is_a_real_zone_and_is_accepted() {
    assert_eq!(Timezone::new("UTC").unwrap().value(), "UTC");
}

#[test]
fn surrounding_whitespace_is_trimmed() {
    assert_eq!(
        Timezone::new("  America/New_York  ").unwrap().value(),
        "America/New_York"
    );
}

#[test]
fn a_zone_that_merely_looks_like_one_is_rejected() {
    assert!(matches!(
        Timezone::new("Foo/Bar"),
        Err(DomainError::InvalidInput(_))
    ));
    assert!(matches!(
        Timezone::new("Europe/Atlantis"),
        Err(DomainError::InvalidInput(_))
    ));
}

#[test]
fn an_empty_zone_is_rejected() {
    assert!(matches!(
        Timezone::new(""),
        Err(DomainError::InvalidInput(_))
    ));
}

#[test]
fn a_stored_zone_that_cannot_be_resolved_is_refused_on_load() {
    assert!(Timezone::from_persistence("Foo/Bar").is_err());
    assert!(Timezone::from_persistence("Europe/Warsaw").is_ok());
}
