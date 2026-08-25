use domain::entry::Mood;
use domain::errors::DomainError;

#[test]
fn mood_values_map_to_expected_scale() {
    assert_eq!(Mood::Awful.value(), 1);
    assert_eq!(Mood::Bad.value(), 2);
    assert_eq!(Mood::Meh.value(), 3);
    assert_eq!(Mood::Good.value(), 4);
    assert_eq!(Mood::Rad.value(), 5);
}

#[test]
fn mood_ordering_reflects_scale() {
    assert!(Mood::Awful < Mood::Bad);
    assert!(Mood::Bad < Mood::Meh);
    assert!(Mood::Meh < Mood::Good);
    assert!(Mood::Good < Mood::Rad);
}

#[test]
fn valid_values_convert_to_mood() {
    for value in 1..=5u8 {
        let mood = Mood::try_from(value);
        assert!(mood.is_ok());
        assert_eq!(mood.unwrap().value(), value);
    }
}

#[test]
fn zero_is_invalid_mood_value() {
    let result = Mood::try_from(0u8);
    assert!(matches!(result, Err(DomainError::InvalidInput(_))));
}

#[test]
fn six_is_invalid_mood_value() {
    let result = Mood::try_from(6u8);
    assert!(matches!(result, Err(DomainError::InvalidInput(_))));
}
