use domain::entry::Date;
use domain::metric::{DailyMetric, MetricValue, Source, Steps};
use domain::provider::ProviderName;
use domain::user::UserId;

fn a_date() -> Date {
    Date::from_persistence("2026-08-26".parse().unwrap())
}

fn steps(count: u32, source: Source) -> DailyMetric {
    DailyMetric::new(
        UserId::generate(),
        a_date(),
        MetricValue::Steps(Steps::new(count).unwrap()),
        source,
    )
}

fn a_provider() -> Source {
    Source::Provider(ProviderName::new("healthkit").unwrap())
}

#[test]
fn a_manual_count_supersedes_one_a_provider_reported() {
    let manual = steps(8_412, Source::Manual);
    let imported = steps(8_000, a_provider());

    assert!(manual.supersedes(&imported));
}

#[test]
fn a_provider_does_not_overwrite_a_count_the_user_stated() {
    let manual = steps(8_412, Source::Manual);
    let imported = steps(8_000, a_provider());

    assert!(!imported.supersedes(&manual));
}

#[test]
fn a_later_import_replaces_an_earlier_one() {
    let earlier = steps(8_000, a_provider());
    let later = steps(8_412, a_provider());

    assert!(later.supersedes(&earlier));
}

#[test]
fn a_correction_replaces_the_count_the_user_stated_before() {
    let stated = steps(8_000, Source::Manual);
    let corrected = steps(8_412, Source::Manual);

    assert!(corrected.supersedes(&stated));
}
