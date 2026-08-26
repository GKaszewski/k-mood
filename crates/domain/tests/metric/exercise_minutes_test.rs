use domain::metric::ExerciseMinutes;

#[test]
fn more_exercise_than_a_day_holds_is_rejected() {
    assert!(ExerciseMinutes::new(1_441).is_err());
}

#[test]
fn the_boundaries_of_a_days_exercise_are_valid() {
    assert_eq!(ExerciseMinutes::new(0).unwrap().value(), 0);
    assert_eq!(ExerciseMinutes::new(1_440).unwrap().value(), 1_440);
}
