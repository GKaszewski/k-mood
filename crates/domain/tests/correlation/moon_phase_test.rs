use domain::entry::Date;
use domain::moon::MoonPhase;

fn on(day: &str) -> MoonPhase {
    MoonPhase::on(&Date::from_persistence(day.parse().unwrap()))
}

#[test]
fn a_total_solar_eclipse_happens_at_a_new_moon() {
    for day in ["2017-08-21", "2024-04-08"] {
        let illumination = on(day).illumination();

        assert!(
            illumination < 0.02,
            "{day} should be dark, got {illumination}"
        );
    }
}

#[test]
fn a_total_lunar_eclipse_happens_at_a_full_moon() {
    for day in ["2000-01-21", "2018-01-31", "2019-01-21", "2022-05-16"] {
        let illumination = on(day).illumination();

        assert!(
            illumination > 0.98,
            "{day} should be full, got {illumination}"
        );
    }
}

#[test]
fn illumination_never_leaves_its_bounds() {
    let mut day = Date::from_persistence("2024-01-01".parse().unwrap());

    for _ in 0..800 {
        let illumination = MoonPhase::on(&day).illumination();
        assert!((0.0..=1.0).contains(&illumination), "got {illumination}");
        day = day.next();
    }
}

#[test]
fn the_same_illumination_is_told_apart_by_waxing_and_waning() {
    let waxing = on("2024-04-15");
    let waning = on("2024-04-30");

    assert!((waxing.illumination() - 0.5).abs() < 0.15);
    assert!((waning.illumination() - 0.5).abs() < 0.15);
    assert_eq!(waxing.name(), "First Quarter");
    assert_eq!(waning.name(), "Last Quarter");
}

#[test]
fn the_extremes_are_named_for_what_they_are() {
    assert_eq!(on("2024-04-08").name(), "New");
    assert_eq!(on("2022-05-16").name(), "Full");
}
