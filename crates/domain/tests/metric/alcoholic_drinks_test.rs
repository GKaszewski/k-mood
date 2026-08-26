use domain::metric::AlcoholicDrinks;

#[test]
fn a_count_of_drinks_no_one_survives_is_rejected() {
    assert!(AlcoholicDrinks::new(61).is_err());
}

#[test]
fn the_boundaries_of_a_days_drinking_are_valid() {
    assert_eq!(AlcoholicDrinks::new(0).unwrap().value(), 0);
    assert_eq!(AlcoholicDrinks::new(60).unwrap().value(), 60);
}
