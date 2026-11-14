use domain::comparison::{DayWeather, PeriodSummary};
use domain::weather::Condition;

use super::period_fixtures::{day_on, october_input, weather};

#[test]
fn a_day_of_mixed_weather_takes_the_most_frequent_condition() {
    let readings = [
        weather(Condition::Rain, 10.0),
        weather(Condition::Clear, 20.0),
        weather(Condition::Rain, 14.0),
    ];

    assert_eq!(
        DayWeather::of(&readings),
        Some((Condition::Rain, 44.0 / 3.0))
    );
}

#[test]
fn a_tie_goes_to_the_condition_listed_first() {
    let readings = [
        weather(Condition::Rain, 10.0),
        weather(Condition::Clear, 10.0),
    ];

    let (condition, _) = DayWeather::of(&readings).unwrap();

    assert_eq!(condition, Condition::Clear);
}

#[test]
fn a_day_without_readings_has_no_weather() {
    assert_eq!(DayWeather::of(&[]), None);
}

#[test]
fn the_summary_groups_moods_and_temperatures_by_day_condition() {
    let mut rainy = day_on(1, 2);
    rainy.weather = vec![weather(Condition::Rain, 8.0)];
    let mut sunny = day_on(2, 5);
    sunny.weather = vec![weather(Condition::Clear, 22.0)];
    let unrecorded = day_on(3, 3);

    let summary = PeriodSummary::of(&october_input(vec![rainy, sunny, unrecorded]));

    assert_eq!(summary.days_with_weather(), 2);
    assert_eq!(summary.condition_days(Condition::Rain), 1);
    assert_eq!(summary.moods_in(Condition::Clear), vec![5.0]);
    assert_eq!(summary.temperatures(), vec![8.0, 22.0]);
}
