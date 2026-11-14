use std::collections::{BTreeMap, BTreeSet};

use crate::song::{ArtistName, SongTitle};

use super::{PeriodDay, Ranked};

#[derive(Debug, Clone, PartialEq)]
pub struct MusicSummary {
    pub entries_with_song: usize,
    pub artists: Vec<Ranked<ArtistName>>,
    pub songs: Vec<Ranked<(ArtistName, SongTitle)>>,
    artist_plays: BTreeMap<ArtistName, usize>,
    top: usize,
}

impl MusicSummary {
    pub fn of(days: &[PeriodDay], top: usize) -> Self {
        let plays: Vec<_> = days.iter().flat_map(|day| &day.songs).collect();
        let entries_with_song = plays.len();
        let artist_counts = tally(plays.iter().map(|song| song.artist().clone()));
        let song_counts = tally(
            plays
                .iter()
                .map(|song| (song.artist().clone(), song.title().clone())),
        );
        let artist_plays = artist_counts.clone();

        Self {
            entries_with_song,
            artists: rank(artist_counts, entries_with_song, top),
            songs: rank(song_counts, entries_with_song, top),
            artist_plays,
            top,
        }
    }

    pub fn all_artists(&self) -> BTreeSet<&ArtistName> {
        self.artist_plays.keys().collect()
    }

    pub fn plays_of(&self, artist: &ArtistName) -> usize {
        self.artist_plays.get(artist).copied().unwrap_or(0)
    }

    pub fn top(&self) -> usize {
        self.top
    }
}

fn tally<T: Ord>(items: impl Iterator<Item = T>) -> BTreeMap<T, usize> {
    let mut counts = BTreeMap::new();
    for item in items {
        *counts.entry(item).or_insert(0) += 1;
    }
    counts
}

fn rank<T: Ord>(counts: BTreeMap<T, usize>, total: usize, top: usize) -> Vec<Ranked<T>> {
    let mut ranked: Vec<_> = counts
        .into_iter()
        .map(|(item, count)| Ranked {
            item,
            count,
            share: count as f64 / total as f64,
        })
        .collect();
    ranked.sort_by_key(|entry| std::cmp::Reverse(entry.count));
    ranked.truncate(top);
    ranked
}
