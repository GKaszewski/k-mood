use domain::cycle::CycleCalendar;
use domain::entry::Date;

fn on(day: &str) -> Date {
    Date::from_persistence(day.parse().unwrap())
}

fn calendar(starts: &[&str]) -> CycleCalendar {
    CycleCalendar::new(starts.iter().map(|day| on(day)).collect())
}

#[test]
fn the_day_a_cycle_starts_is_its_first_day() {
    let position = calendar(&["2026-01-01"])
        .position_on(&on("2026-01-01"))
        .unwrap();

    assert_eq!(position.day().value(), 1);
    assert!(position.progress().abs() < f64::EPSILON);
}

#[test]
fn a_date_before_anything_was_recorded_has_no_place_in_a_cycle() {
    assert!(
        calendar(&["2026-02-01"])
            .position_on(&on("2026-01-15"))
            .is_none()
    );
}

#[test]
fn nothing_recorded_at_all_means_nothing_derived() {
    assert!(calendar(&[]).position_on(&on("2026-01-15")).is_none());
}

#[test]
fn a_closed_cycle_is_measured_by_its_own_length() {
    let starts = calendar(&["2026-01-01", "2026-01-25"]);

    let halfway = starts.position_on(&on("2026-01-13")).unwrap();

    assert_eq!(halfway.day().value(), 13);
    assert!(
        (halfway.progress() - 0.5).abs() < 1e-12,
        "twelve days into a twenty-four day cycle, got {}",
        halfway.progress()
    );
}

#[test]
fn the_cycle_still_running_is_measured_against_the_usual_length() {
    let starts = calendar(&["2026-01-01", "2026-01-21", "2026-02-10", "2026-03-02"]);

    let open = starts.position_on(&on("2026-03-12")).unwrap();

    assert_eq!(open.day().value(), 11);
    assert!(
        (open.progress() - 0.5).abs() < 1e-12,
        "ten days into a twenty-day cycle, got {}",
        open.progress()
    );
}

#[test]
fn with_nothing_to_go_on_the_usual_length_is_assumed() {
    let starts = calendar(&["2026-01-01"]);

    let position = starts.position_on(&on("2026-01-15")).unwrap();

    assert_eq!(position.day().value(), 15);
    assert!(
        (position.progress() - 0.5).abs() < 1e-12,
        "fourteen days into an assumed twenty-eight day cycle, got {}",
        position.progress()
    );
}

#[test]
fn correcting_the_start_moves_every_day_that_depended_on_it() {
    let before = calendar(&["2026-01-01"])
        .position_on(&on("2026-01-10"))
        .unwrap();
    let after = calendar(&["2026-01-03"])
        .position_on(&on("2026-01-10"))
        .unwrap();

    assert_eq!(before.day().value(), 10);
    assert_eq!(after.day().value(), 8);
}

#[test]
fn a_gap_too_long_to_be_one_cycle_means_a_start_was_missed() {
    let position = calendar(&["2026-01-01"]).position_on(&on("2026-06-01"));

    assert!(
        position.is_none(),
        "a hundred and fifty days is a missing record, not a long cycle"
    );
}

#[test]
fn the_order_and_repetition_of_what_was_recorded_does_not_matter() {
    let tidy = calendar(&["2026-01-01", "2026-01-25"]);
    let messy = calendar(&["2026-01-25", "2026-01-01", "2026-01-25"]);

    let from_tidy = tidy.position_on(&on("2026-01-13")).unwrap();
    let from_messy = messy.position_on(&on("2026-01-13")).unwrap();

    assert_eq!(from_tidy.day().value(), from_messy.day().value());
    assert!((from_tidy.progress() - from_messy.progress()).abs() < f64::EPSILON);
}

#[test]
fn the_last_day_of_a_cycle_has_run_its_whole_course() {
    let starts = calendar(&["2026-01-01", "2026-01-29"]);

    let last = starts.position_on(&on("2026-01-28")).unwrap();

    assert_eq!(last.day().value(), 28);
    assert!(
        (last.progress() - 27.0 / 28.0).abs() < 1e-12,
        "got {}",
        last.progress()
    );
}

#[test]
fn progress_never_runs_past_the_end_of_a_cycle() {
    let starts = calendar(&["2026-01-01", "2026-01-15", "2026-01-29"]);

    let overdue = starts.position_on(&on("2026-02-28")).unwrap();

    assert_eq!(overdue.day().value(), 31);
    assert!(
        (overdue.progress() - 1.0).abs() < f64::EPSILON,
        "a late cycle sits at its end rather than beyond it, got {}",
        overdue.progress()
    );
}
