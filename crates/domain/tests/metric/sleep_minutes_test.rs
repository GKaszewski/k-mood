use domain::metric::SleepMinutes;

#[test]
fn a_night_longer_than_a_day_is_rejected() {
    assert!(SleepMinutes::new(1_441).is_err());
}

#[test]
fn the_boundaries_of_a_nights_sleep_are_valid() {
    assert_eq!(SleepMinutes::new(0).unwrap().value(), 0);
    assert_eq!(SleepMinutes::new(1_440).unwrap().value(), 1_440);
}
