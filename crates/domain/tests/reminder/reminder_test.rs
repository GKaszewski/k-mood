use chrono::{NaiveTime, Weekday};

use domain::reminder::{DaySchedule, Reminder};
use domain::user::UserId;

#[test]
fn new_reminder_is_enabled() {
    let schedule = DaySchedule::every_day_at(NaiveTime::from_hms_opt(20, 0, 0).unwrap());
    let reminder = Reminder::new(UserId::generate(), schedule);

    assert!(reminder.is_enabled());
}

#[test]
fn disable_and_enable_toggle_state() {
    let schedule = DaySchedule::every_day_at(NaiveTime::from_hms_opt(20, 0, 0).unwrap());
    let mut reminder = Reminder::new(UserId::generate(), schedule);

    reminder.disable();
    assert!(!reminder.is_enabled());

    reminder.enable();
    assert!(reminder.is_enabled());
}

#[test]
fn every_day_at_sets_all_days() {
    let time = NaiveTime::from_hms_opt(20, 0, 0).unwrap();
    let schedule = DaySchedule::every_day_at(time);

    for day in [
        Weekday::Mon,
        Weekday::Tue,
        Weekday::Wed,
        Weekday::Thu,
        Weekday::Fri,
        Weekday::Sat,
        Weekday::Sun,
    ] {
        assert_eq!(schedule.time_for(day), Some(time));
    }
}

#[test]
fn empty_schedule_has_no_days_set() {
    let schedule = DaySchedule::new();
    assert!(!schedule.has_any_day_set());
}

#[test]
fn set_time_for_specific_day() {
    let mut schedule = DaySchedule::new();
    let time = NaiveTime::from_hms_opt(20, 0, 0).unwrap();

    schedule.set_time(Weekday::Mon, Some(time));

    assert_eq!(schedule.time_for(Weekday::Mon), Some(time));
    assert_eq!(schedule.time_for(Weekday::Tue), None);
    assert!(schedule.has_any_day_set());
}

#[test]
fn update_schedule_replaces_entire_schedule() {
    let old = DaySchedule::every_day_at(NaiveTime::from_hms_opt(20, 0, 0).unwrap());
    let new = DaySchedule::every_day_at(NaiveTime::from_hms_opt(9, 0, 0).unwrap());
    let mut reminder = Reminder::new(UserId::generate(), old);

    reminder.update_schedule(new);

    let expected = NaiveTime::from_hms_opt(9, 0, 0).unwrap();
    assert_eq!(reminder.schedule().time_for(Weekday::Mon), Some(expected));
}

#[test]
fn clearing_a_set_day_removes_it() {
    let mut schedule = DaySchedule::every_day_at(NaiveTime::from_hms_opt(20, 0, 0).unwrap());
    assert!(schedule.time_for(Weekday::Mon).is_some());

    schedule.set_time(Weekday::Mon, None);
    assert!(schedule.time_for(Weekday::Mon).is_none());
    assert!(schedule.has_any_day_set());
}
