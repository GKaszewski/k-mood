use chrono::{DateTime, Duration, NaiveTime, TimeZone, Utc, Weekday};

use domain::reminder::{DaySchedule, Reminder};
use domain::user::{Timezone, UserId};

const GRACE: Duration = Duration::minutes(30);

fn warsaw() -> Timezone {
    Timezone::new("Europe/Warsaw").unwrap()
}

fn at(time: &str) -> NaiveTime {
    NaiveTime::parse_from_str(time, "%H:%M").unwrap()
}

fn instant(text: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(text)
        .unwrap()
        .with_timezone(&Utc)
}

fn every_day_at(time: &str) -> Reminder {
    Reminder::new(UserId::generate(), DaySchedule::every_day_at(at(time)))
}

#[test]
fn a_reminder_is_due_once_its_local_time_has_arrived() {
    let reminder = every_day_at("20:00");
    let just_after_eight_in_warsaw = instant("2026-08-27T18:01:00Z");

    assert!(
        reminder
            .occurrence_reached(just_after_eight_in_warsaw, &warsaw(), GRACE)
            .is_some()
    );
}

#[test]
fn a_reminder_is_not_due_before_its_local_time() {
    let reminder = every_day_at("20:00");
    let half_past_seven_in_warsaw = instant("2026-08-27T17:30:00Z");

    assert!(
        reminder
            .occurrence_reached(half_past_seven_in_warsaw, &warsaw(), GRACE)
            .is_none()
    );
}

#[test]
fn the_same_occurrence_is_never_due_twice() {
    let mut reminder = every_day_at("20:00");
    let first_sweep = instant("2026-08-27T18:01:00Z");

    let occurrence = reminder
        .occurrence_reached(first_sweep, &warsaw(), GRACE)
        .expect("the reminder should be due on the first sweep");

    reminder.mark_sent(occurrence);

    for minute in 2..=25 {
        let later = instant(&format!("2026-08-27T18:{minute:02}:00Z"));

        assert!(
            reminder
                .occurrence_reached(later, &warsaw(), GRACE)
                .is_none(),
            "a reminder already sent must not be sent again at 18:{minute:02}"
        );
    }
}

#[test]
fn the_next_days_occurrence_is_due_again() {
    let mut reminder = every_day_at("20:00");
    let today = instant("2026-08-27T18:01:00Z");

    let occurrence = reminder
        .occurrence_reached(today, &warsaw(), GRACE)
        .unwrap();
    reminder.mark_sent(occurrence);

    let tomorrow = instant("2026-08-28T18:01:00Z");

    assert!(
        reminder
            .occurrence_reached(tomorrow, &warsaw(), GRACE)
            .is_some(),
        "a new day is a new occurrence"
    );
}

#[test]
fn an_occurrence_late_yesterday_is_still_reachable_after_local_midnight() {
    let schedule = {
        let mut schedule = DaySchedule::new();
        schedule.set_time(Weekday::Thu, Some(at("23:50")));
        schedule
    };
    let reminder = Reminder::new(UserId::generate(), schedule);

    let ten_past_midnight_on_friday = instant("2026-08-27T22:10:00Z");

    assert!(
        reminder
            .occurrence_reached(ten_past_midnight_on_friday, &warsaw(), GRACE)
            .is_some(),
        "a reminder scheduled just before midnight must survive the day boundary"
    );
}

#[test]
fn an_occurrence_older_than_the_grace_window_is_not_resurrected() {
    let reminder = every_day_at("08:00");
    let late_that_evening = instant("2026-08-27T19:00:00Z");

    assert!(
        reminder
            .occurrence_reached(late_that_evening, &warsaw(), GRACE)
            .is_none(),
        "a worker starting late must not fire this morning's reminder"
    );
}

#[test]
fn a_disabled_reminder_is_never_due() {
    let mut reminder = every_day_at("20:00");
    reminder.disable();

    assert!(
        reminder
            .occurrence_reached(instant("2026-08-27T18:01:00Z"), &warsaw(), GRACE)
            .is_none()
    );
}

#[test]
fn a_day_with_no_time_set_is_never_due() {
    let schedule = {
        let mut schedule = DaySchedule::new();
        schedule.set_time(Weekday::Mon, Some(at("20:00")));
        schedule
    };
    let reminder = Reminder::new(UserId::generate(), schedule);

    let a_thursday = instant("2026-08-27T18:01:00Z");

    assert!(
        reminder
            .occurrence_reached(a_thursday, &warsaw(), GRACE)
            .is_none()
    );
}

#[test]
fn the_scheduled_instant_is_resolved_in_the_users_own_zone() {
    let reminder = every_day_at("20:00");
    let warsaw_evening = instant("2026-08-27T18:01:00Z");

    let occurrence = reminder
        .occurrence_reached(warsaw_evening, &warsaw(), GRACE)
        .unwrap();

    assert_eq!(
        occurrence.scheduled_for(),
        Utc.with_ymd_and_hms(2026, 8, 27, 18, 0, 0).unwrap(),
        "8 PM in Warsaw in August is 18:00Z"
    );
}
