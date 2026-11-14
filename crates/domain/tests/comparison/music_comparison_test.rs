use domain::comparison::PeriodInput;
use domain::song::ArtistName;

use super::comparison_fixtures::{compare, october, september};
use super::period_fixtures::song;

const MEH: u8 = 3;

fn artist(name: &str) -> ArtistName {
    ArtistName::new(name).unwrap()
}

fn with_songs(mut input: PeriodInput, plays: &[(&str, &str)]) -> PeriodInput {
    input.days[0].songs = plays
        .iter()
        .map(|(title, name)| song(title, name))
        .collect();
    input
}

#[test]
fn artists_only_in_one_period_are_listed() {
    let a = with_songs(
        september(&[MEH]),
        &[
            ("One", "Delta"),
            ("Two", "Alpha"),
            ("Three", "Alpha"),
            ("Four", "Beta"),
        ],
    );
    let b = with_songs(october(&[MEH]), &[("Five", "Beta"), ("Six", "Gamma")]);

    let comparison = compare(a, b);

    assert_eq!(
        comparison.music().only_in_a(),
        &[artist("Alpha"), artist("Delta")]
    );
    assert_eq!(comparison.music().only_in_b(), &[artist("Gamma")]);
}

#[test]
fn artists_only_in_one_period_are_capped_at_the_top_count() {
    let mut a = with_songs(
        september(&[MEH]),
        &[("One", "Delta"), ("Two", "Alpha"), ("Three", "Alpha")],
    );
    a.top_music_count = 1;
    let mut b = october(&[MEH]);
    b.top_music_count = 1;

    let comparison = compare(a, b);

    assert_eq!(comparison.music().only_in_a(), &[artist("Alpha")]);
    assert!(comparison.music().only_in_b().is_empty());
}
