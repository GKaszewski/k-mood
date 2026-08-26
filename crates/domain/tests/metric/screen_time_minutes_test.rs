use domain::metric::ScreenTimeMinutes;

#[test]
fn more_screen_time_than_a_day_holds_is_rejected() {
    assert!(ScreenTimeMinutes::new(1_441).is_err());
}

#[test]
fn the_boundaries_of_a_days_screen_time_are_valid() {
    assert_eq!(ScreenTimeMinutes::new(0).unwrap().value(), 0);
    assert_eq!(ScreenTimeMinutes::new(1_440).unwrap().value(), 1_440);
}
