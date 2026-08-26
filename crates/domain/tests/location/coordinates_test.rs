use domain::location::{Coordinates, Latitude, Longitude};

#[test]
fn coordinates_are_built_from_valid_bounds() {
    let coordinates = Coordinates::new(52.2297, 21.0122).unwrap();

    assert!((coordinates.latitude().value() - 52.2297).abs() < f64::EPSILON);
    assert!((coordinates.longitude().value() - 21.0122).abs() < f64::EPSILON);
}

#[test]
fn the_poles_and_the_antimeridian_are_valid() {
    assert!(Latitude::new(90.0).is_ok());
    assert!(Latitude::new(-90.0).is_ok());
    assert!(Longitude::new(180.0).is_ok());
    assert!(Longitude::new(-180.0).is_ok());
}

#[test]
fn a_latitude_beyond_the_poles_is_rejected() {
    assert!(Latitude::new(90.1).is_err());
    assert!(Latitude::new(-90.1).is_err());
}

#[test]
fn a_longitude_beyond_the_antimeridian_is_rejected() {
    assert!(Longitude::new(180.1).is_err());
    assert!(Longitude::new(-180.1).is_err());
}

#[test]
fn a_coordinate_that_is_not_a_number_is_rejected() {
    assert!(Latitude::new(f64::NAN).is_err());
    assert!(Longitude::new(f64::INFINITY).is_err());
}
