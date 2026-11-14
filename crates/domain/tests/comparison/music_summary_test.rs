use domain::comparison::{MusicSummary, PeriodSummary};
use domain::song::ArtistName;

use super::period_fixtures::{day_on, october_input, song};

const TOP_TWO: usize = 2;

fn artist(name: &str) -> ArtistName {
    ArtistName::new(name).unwrap()
}

fn day_with_songs(day_of_month: u32, songs: &[(&str, &str)]) -> domain::comparison::PeriodDay {
    let mut day = day_on(day_of_month, 3);
    day.songs = songs
        .iter()
        .map(|(title, name)| song(title, name))
        .collect();
    day
}

#[test]
fn music_ranks_by_share_of_entries_with_a_song() {
    let days = vec![
        day_with_songs(1, &[("One", "Beta"), ("Two", "Alpha")]),
        day_with_songs(2, &[("One", "Beta"), ("Three", "Beta")]),
    ];

    let music = MusicSummary::of(&days, TOP_TWO);

    assert_eq!(music.entries_with_song, 4);
    assert_eq!(music.artists[0].item, artist("Beta"));
    assert_eq!(music.artists[0].count, 3);
    assert_eq!(music.artists[0].share, 0.75);
    assert_eq!(music.songs[0].item.1.value(), "One");
    assert_eq!(music.songs[0].count, 2);
}

#[test]
fn ties_are_ordered_by_name() {
    let days = vec![day_with_songs(1, &[("X", "Zed"), ("Y", "Abe")])];

    let music = MusicSummary::of(&days, TOP_TWO);

    assert_eq!(music.artists[0].item, artist("Abe"));
}

#[test]
fn music_keeps_only_the_top_n_but_remembers_every_artist() {
    let days = vec![day_with_songs(
        1,
        &[("A", "One"), ("B", "Two"), ("C", "Three"), ("D", "Three")],
    )];

    let music = MusicSummary::of(&days, TOP_TWO);

    assert_eq!(music.artists.len(), TOP_TWO);
    assert_eq!(music.songs.len(), TOP_TWO);
    assert_eq!(music.all_artists().len(), 3);
}

#[test]
fn a_period_without_songs_has_no_music() {
    let summary = PeriodSummary::of(&october_input(vec![day_on(1, 3)]));

    assert_eq!(summary.music().entries_with_song, 0);
    assert!(summary.music().artists.is_empty());
}

#[test]
fn the_period_input_decides_how_many_ranked_items_the_summary_keeps() {
    let mut input = october_input(vec![day_with_songs(
        1,
        &[("A", "One"), ("B", "Two"), ("C", "Three")],
    )]);
    input.top_music_count = 1;

    let summary = PeriodSummary::of(&input);

    assert_eq!(summary.music().artists.len(), 1);
    assert_eq!(summary.music().all_artists().len(), 3);
}
