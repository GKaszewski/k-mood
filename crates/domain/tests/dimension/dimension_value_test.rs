use domain::activity::ActivityId;
use domain::dimension::{DimensionKind, DimensionValue};

#[test]
fn activities_are_deduplicated_and_sorted() {
    let first = ActivityId::generate();
    let second = ActivityId::generate();

    let value = DimensionValue::activities(vec![second.clone(), first.clone(), second.clone()]);

    let DimensionValue::Activities(ids) = value else {
        panic!("expected an activities dimension");
    };

    assert_eq!(ids.len(), 2);
    assert!(ids[0] <= ids[1]);
}

#[test]
fn every_value_reports_its_own_kind() {
    assert_eq!(
        DimensionValue::activities(vec![]).kind(),
        DimensionKind::Activities
    );
    assert_eq!(DimensionValue::Photos(vec![]).kind(), DimensionKind::Photos);
    assert_eq!(
        DimensionValue::VoiceMemos(vec![]).kind(),
        DimensionKind::VoiceMemos
    );
}
