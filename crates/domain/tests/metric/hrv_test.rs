use domain::metric::Hrv;

#[test]
fn a_variability_of_zero_is_rejected() {
    assert!(Hrv::new(0).is_err());
}

#[test]
fn a_variability_beyond_any_recorded_reading_is_rejected() {
    assert!(Hrv::new(301).is_err());
}

#[test]
fn the_boundaries_of_variability_are_valid() {
    assert_eq!(Hrv::new(1).unwrap().value(), 1);
    assert_eq!(Hrv::new(300).unwrap().value(), 300);
}
