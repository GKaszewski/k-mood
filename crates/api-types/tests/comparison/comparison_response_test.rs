use std::collections::{BTreeMap, BTreeSet};

use api_types::responses::ComparisonResponse;
use application::comparison::use_cases::compare_periods::PeriodComparison;
use chrono::NaiveDate;
use domain::activity::{Activity, ActivityId, ActivityName, CategoryName};
use domain::comparison::{
    ComparedSpans, Comparison, ComparisonRules, MaximumSpan, PeriodDay, PeriodInput, PeriodSummary,
};
use domain::entry::{Date, DateSpan, DayMood, Mood};
use domain::user::UserId;

const TODAY: &str = "2026-11-14";
const FIRST_DAY: &str = "2026-10-01";
const LAST_DAY: &str = "2026-10-31";
const MAXIMUM_DAYS: i64 = 366;
const MINIMUM_SAMPLE: usize = 10;
const FALSE_DISCOVERY_RATE: f64 = 0.10;
const TOP_MUSIC: usize = 5;
const RULES: ComparisonRules = ComparisonRules {
    minimum_sample_size: MINIMUM_SAMPLE,
    false_discovery_rate: FALSE_DISCOVERY_RATE,
};

fn date(text: &str) -> Date {
    Date::from_persistence(NaiveDate::parse_from_str(text, "%Y-%m-%d").unwrap())
}

fn day_with(activity: &ActivityId) -> PeriodDay {
    PeriodDay {
        date: date(FIRST_DAY),
        mood: DayMood::of(&[Mood::Good]).unwrap(),
        entries: 1,
        activities: BTreeSet::from([activity.clone()]),
        weather: Vec::new(),
        songs: Vec::new(),
        cycle: None,
    }
}

fn october_input(activity: &ActivityId) -> PeriodInput {
    let span = DateSpan::new(date(FIRST_DAY), date(LAST_DAY))
        .unwrap()
        .clipped_to(date(TODAY))
        .unwrap();
    PeriodInput {
        span,
        days: vec![day_with(activity)],
        metrics: BTreeMap::new(),
        tracks_cycle: false,
        top_music_count: TOP_MUSIC,
    }
}

fn overlapping_comparison(
    activity: &ActivityId,
    labels: BTreeMap<ActivityId, Activity>,
) -> PeriodComparison {
    let input = october_input(activity);
    let spans = ComparedSpans::new(
        *input.span.span(),
        *input.span.span(),
        MaximumSpan::of_days(MAXIMUM_DAYS),
    )
    .unwrap();
    let comparison = Comparison::between(
        PeriodSummary::of(&input),
        PeriodSummary::of(&input),
        &spans,
        &[],
        &[],
        RULES,
    );
    PeriodComparison {
        comparison,
        spans: (input.span, input.span),
        activities: labels,
    }
}

fn labelled(activity: &ActivityId) -> BTreeMap<ActivityId, Activity> {
    let named = Activity::from_persistence(
        activity.clone(),
        UserId::generate(),
        ActivityName::new("Running").unwrap(),
        Some(CategoryName::new("Sport").unwrap()),
        false,
        chrono::Utc::now(),
    );
    BTreeMap::from([(activity.clone(), named)])
}

fn json_of(comparison: PeriodComparison) -> serde_json::Value {
    serde_json::to_value(ComparisonResponse::from(comparison)).unwrap()
}

#[test]
fn an_activity_row_lands_in_the_activities_section_untested_when_spans_overlap() {
    let activity = ActivityId::generate();

    let json = json_of(overlapping_comparison(&activity, labelled(&activity)));

    let row = &json["activities"][0];
    assert_eq!(row["key"], "activityShare");
    assert_eq!(row["significance"], "notTested");
    assert_eq!(row["name"], "Running");
    assert_eq!(row["category"], "Sport");
    assert_eq!(row["activityId"], activity.value().to_string());
    assert_eq!(json["spansOverlap"], true);
}

#[test]
fn an_activity_missing_from_the_labels_is_named_unknown_not_refused() {
    let activity = ActivityId::generate();

    let json = json_of(overlapping_comparison(&activity, BTreeMap::new()));

    assert_eq!(json["activities"][0]["name"], "Unknown activity");
    assert!(json["activities"][0]["category"].is_null());
}

#[test]
fn rows_are_routed_to_their_sections() {
    let activity = ActivityId::generate();

    let json = json_of(overlapping_comparison(&activity, labelled(&activity)));

    assert_eq!(json["overview"][0]["key"], "daysInSpan");
    assert_eq!(json["mood"]["rows"][0]["key"], "averageMood");
    assert_eq!(
        json["mood"]["distribution"]["a"].as_array().unwrap().len(),
        5
    );
    assert_eq!(json["a"]["from"], FIRST_DAY);
    assert_eq!(json["a"]["clipped"], false);
    assert!(json["cycle"].is_null());
    assert_eq!(json["trend"]["granularity"], "day");
}
