use domain::trend::{Granularity, WeekStart};

#[test]
fn a_month_is_drawn_in_days() {
    assert_eq!(Granularity::for_days(31), Granularity::Day);
}

#[test]
fn sixty_three_days_are_drawn_in_weeks() {
    assert_eq!(Granularity::for_days(63), Granularity::Week);
}

#[test]
fn a_leap_year_is_drawn_in_weeks() {
    assert_eq!(Granularity::for_days(366), Granularity::Week);
}

#[test]
fn two_years_are_drawn_in_months() {
    assert_eq!(Granularity::for_days(730), Granularity::Month);
}

#[test]
fn week_start_defaults_to_monday() {
    assert_eq!(WeekStart::default(), WeekStart::Monday);
}

#[test]
fn week_start_is_parsed_from_lowercase_english_names() {
    assert_eq!(WeekStart::from_name("sunday"), Some(WeekStart::Sunday));
    assert_eq!(
        WeekStart::from_name("wednesday"),
        Some(WeekStart::Wednesday)
    );
    assert_eq!(WeekStart::from_name("someday"), None);
}

#[test]
fn a_single_period_of_a_month_is_drawn_in_days() {
    assert_eq!(Granularity::for_single_period(31), Granularity::Day);
}

#[test]
fn a_single_period_just_over_a_month_is_drawn_in_weeks() {
    assert_eq!(Granularity::for_single_period(32), Granularity::Week);
}

#[test]
fn a_rolling_year_of_366_days_is_drawn_in_weeks() {
    assert_eq!(Granularity::for_single_period(366), Granularity::Week);
}

#[test]
fn a_single_period_over_a_rolling_366_day_year_is_drawn_in_months() {
    assert_eq!(Granularity::for_single_period(367), Granularity::Month);
}
