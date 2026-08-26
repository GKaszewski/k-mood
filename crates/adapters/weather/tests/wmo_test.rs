use domain::weather::Condition;

use weather::condition_for;

#[test]
fn the_documented_wmo_codes_map_onto_the_domains_vocabulary() {
    let expected = [
        (0, Condition::Clear),
        (1, Condition::Clear),
        (2, Condition::Cloudy),
        (3, Condition::Cloudy),
        (45, Condition::Fog),
        (48, Condition::Fog),
        (51, Condition::Drizzle),
        (55, Condition::Drizzle),
        (61, Condition::Rain),
        (65, Condition::Rain),
        (66, Condition::Rain),
        (80, Condition::Rain),
        (82, Condition::Rain),
        (71, Condition::Snow),
        (75, Condition::Snow),
        (77, Condition::Snow),
        (85, Condition::Snow),
        (86, Condition::Snow),
        (95, Condition::Thunderstorm),
        (99, Condition::Thunderstorm),
    ];

    for (code, condition) in expected {
        assert_eq!(
            condition_for(code),
            Some(condition),
            "wmo code {code} should be {condition:?}"
        );
    }
}

#[test]
fn a_code_the_provider_never_documents_maps_to_nothing() {
    for code in [4, 20, 44, 50, 60, 70, 90, 100, 255] {
        assert_eq!(
            condition_for(code),
            None,
            "wmo code {code} is not documented"
        );
    }
}

#[test]
fn every_condition_in_the_vocabulary_is_reachable_from_some_code() {
    let reached: Vec<Condition> = (0..=99).filter_map(condition_for).collect();

    for condition in Condition::ALL {
        assert!(
            reached.contains(&condition),
            "{condition:?} cannot be produced by any wmo code"
        );
    }
}
