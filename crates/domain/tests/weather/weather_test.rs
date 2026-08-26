use domain::provider::ProviderName;
use domain::weather::{Celsius, Condition, Weather};

fn open_meteo() -> ProviderName {
    ProviderName::new("open-meteo").unwrap()
}

#[test]
fn a_temperature_nowhere_on_earth_reaches_is_rejected() {
    assert!(Celsius::new(-95.0).is_err());
    assert!(Celsius::new(65.0).is_err());
}

#[test]
fn the_coldest_and_hottest_places_on_earth_are_valid() {
    assert!(Celsius::new(-90.0).is_ok());
    assert!(Celsius::new(60.0).is_ok());
}

#[test]
fn a_temperature_that_is_not_a_number_is_rejected() {
    assert!(Celsius::new(f64::NAN).is_err());
    assert!(Celsius::new(f64::INFINITY).is_err());
}

#[test]
fn every_condition_survives_a_round_trip_through_its_name() {
    for condition in Condition::ALL {
        assert_eq!(Condition::from_name(condition.name()), Some(condition));
    }

    assert_eq!(Condition::from_name("raining cats"), None);
}

#[test]
fn the_vocabulary_is_the_one_the_domain_chose_not_the_providers() {
    let names: Vec<&str> = Condition::ALL.iter().map(|c| c.name()).collect();

    assert_eq!(
        names,
        [
            "clear",
            "cloudy",
            "fog",
            "drizzle",
            "rain",
            "snow",
            "thunderstorm"
        ]
    );
}

#[test]
fn weather_is_always_attributed_to_whoever_observed_it() {
    let weather = Weather::new(Condition::Rain, Celsius::new(11.5).unwrap(), open_meteo());

    assert_eq!(weather.condition(), Condition::Rain);
    assert!((weather.temperature().value() - 11.5).abs() < f64::EPSILON);
    assert_eq!(weather.observed_by().value(), "open-meteo");
}
