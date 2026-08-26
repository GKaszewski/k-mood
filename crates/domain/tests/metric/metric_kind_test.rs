use domain::metric::MetricKind;

const EVERY_KIND: [MetricKind; 8] = [
    MetricKind::Steps,
    MetricKind::SleepMinutes,
    MetricKind::AwakeMinutes,
    MetricKind::RestingHeartRate,
    MetricKind::Hrv,
    MetricKind::ExerciseMinutes,
    MetricKind::ScreenTimeMinutes,
    MetricKind::AlcoholicDrinks,
];

fn wire_name(kind: MetricKind) -> &'static str {
    match kind {
        MetricKind::Steps => "steps",
        MetricKind::SleepMinutes => "sleepMinutes",
        MetricKind::AwakeMinutes => "awakeMinutes",
        MetricKind::RestingHeartRate => "restingHeartRate",
        MetricKind::Hrv => "hrv",
        MetricKind::ExerciseMinutes => "exerciseMinutes",
        MetricKind::ScreenTimeMinutes => "screenTimeMinutes",
        MetricKind::AlcoholicDrinks => "alcoholicDrinks",
    }
}

#[test]
fn every_kind_keeps_the_name_clients_and_rows_already_use() {
    for kind in EVERY_KIND {
        assert_eq!(kind.name(), wire_name(kind));
    }
}

#[test]
fn every_name_resolves_back_to_its_kind() {
    for kind in EVERY_KIND {
        assert_eq!(MetricKind::from_name(kind.name()), Some(kind));
    }
}

#[test]
fn a_name_this_build_does_not_know_resolves_to_nothing() {
    assert_eq!(MetricKind::from_name("telepathy"), None);
    assert_eq!(MetricKind::from_name("caffeine"), None);
}
