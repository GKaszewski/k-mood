use domain::metric::Steps;

#[test]
fn a_step_count_beyond_a_days_maximum_is_rejected() {
    assert!(Steps::new(200_001).is_err());
}

#[test]
fn the_boundaries_of_a_days_step_count_are_valid() {
    assert_eq!(Steps::new(0).unwrap().value(), 0);
    assert_eq!(Steps::new(200_000).unwrap().value(), 200_000);
}
