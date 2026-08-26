use domain::metric::AwakeMinutes;

#[test]
fn more_awake_minutes_than_a_day_holds_are_rejected() {
    assert!(AwakeMinutes::new(1_441).is_err());
}

#[test]
fn the_boundaries_of_time_spent_awake_are_valid() {
    assert_eq!(AwakeMinutes::new(0).unwrap().value(), 0);
    assert_eq!(AwakeMinutes::new(1_440).unwrap().value(), 1_440);
}
