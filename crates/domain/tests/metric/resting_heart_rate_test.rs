use domain::metric::RestingHeartRate;

#[test]
fn a_resting_rate_no_living_person_has_is_rejected() {
    assert!(RestingHeartRate::new(24).is_err());
    assert!(RestingHeartRate::new(121).is_err());
}

#[test]
fn the_range_covers_athletes_and_the_unwell() {
    assert_eq!(RestingHeartRate::new(25).unwrap().value(), 25);
    assert_eq!(RestingHeartRate::new(120).unwrap().value(), 120);
}
