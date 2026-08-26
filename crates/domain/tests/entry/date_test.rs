use chrono::{DateTime, FixedOffset};

use domain::entry::Date;
use domain::user::Timezone;

fn instant(text: &str) -> DateTime<FixedOffset> {
    text.parse().unwrap()
}

fn zone(name: &str) -> Timezone {
    Timezone::new(name).unwrap()
}

#[test]
fn an_instant_resolves_to_the_calendar_day_in_the_users_zone() {
    let date = Date::from_instant(
        &instant("2025-03-15T09:00:00+13:00"),
        &zone("Pacific/Auckland"),
    );

    assert_eq!(date.value().to_string(), "2025-03-15");
}

#[test]
fn the_users_zone_decides_the_day_not_the_offset_the_client_sent() {
    let same_moment = instant("2025-03-15T23:00:00+00:00");

    let in_auckland = Date::from_instant(&same_moment, &zone("Pacific/Auckland"));
    let in_warsaw = Date::from_instant(&same_moment, &zone("Europe/Warsaw"));

    assert_eq!(in_auckland.value().to_string(), "2025-03-16");
    assert_eq!(in_warsaw.value().to_string(), "2025-03-16");
}

#[test]
fn an_evening_in_a_far_eastern_zone_is_not_the_utc_day() {
    let evening = instant("2025-03-14T20:00:00+00:00");

    let date = Date::from_instant(&evening, &zone("Pacific/Auckland"));

    assert_eq!(date.value().to_string(), "2025-03-15");
}

#[test]
fn a_day_knows_the_day_before_it() {
    let date = Date::from_instant(&instant("2025-03-01T12:00:00+00:00"), &zone("UTC"));

    assert_eq!(date.previous().value().to_string(), "2025-02-28");
}

#[test]
fn days_between_two_dates_is_their_gap() {
    let earlier = Date::from_instant(&instant("2025-03-01T12:00:00+00:00"), &zone("UTC"));
    let later = Date::from_instant(&instant("2025-03-05T12:00:00+00:00"), &zone("UTC"));

    assert_eq!(later.days_since(&earlier), 4);
    assert_eq!(earlier.days_since(&later), -4);
}
