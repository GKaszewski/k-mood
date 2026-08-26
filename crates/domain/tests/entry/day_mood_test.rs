use domain::entry::{DayMood, Mood};

#[test]
fn a_day_of_one_awful_and_one_rad_sits_in_the_middle() {
    let day = DayMood::of(&[Mood::Awful, Mood::Rad]).unwrap();

    assert!((day.value() - 3.0).abs() < f64::EPSILON);
    assert_eq!(day.rounded(), Mood::Meh);
}

#[test]
fn a_day_with_no_entries_has_no_mood() {
    assert!(DayMood::of(&[]).is_none());
}

#[test]
fn one_bad_day_among_four_middling_ones_still_shows() {
    let mostly_meh = DayMood::of(&[Mood::Meh, Mood::Meh, Mood::Meh, Mood::Meh, Mood::Awful])
        .unwrap()
        .value();
    let all_meh = DayMood::of(&[Mood::Meh, Mood::Meh, Mood::Meh, Mood::Meh, Mood::Meh])
        .unwrap()
        .value();

    assert!(mostly_meh < all_meh);
}

#[test]
fn a_single_entry_is_the_whole_day() {
    let day = DayMood::of(&[Mood::Bad]).unwrap();

    assert!((day.value() - 2.0).abs() < f64::EPSILON);
    assert_eq!(day.rounded(), Mood::Bad);
}

#[test]
fn a_day_exactly_between_two_moods_shows_the_better_one() {
    let day = DayMood::of(&[Mood::Meh, Mood::Good]).unwrap();

    assert!((day.value() - 3.5).abs() < f64::EPSILON);
    assert_eq!(day.rounded(), Mood::Good);
}

#[test]
fn every_whole_mean_rounds_to_its_own_mood() {
    let pairs = [
        (Mood::Awful, Mood::Awful),
        (Mood::Bad, Mood::Bad),
        (Mood::Meh, Mood::Meh),
        (Mood::Good, Mood::Good),
        (Mood::Rad, Mood::Rad),
    ];

    for (logged, expected) in pairs {
        assert_eq!(DayMood::of(&[logged, logged]).unwrap().rounded(), expected);
    }
}
