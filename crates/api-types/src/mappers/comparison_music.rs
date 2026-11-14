use domain::comparison::{MusicComparison, MusicSummary, Ranked};
use domain::song::{ArtistName, SongTitle};

use crate::responses::{
    MusicComparisonResponse, MusicResponse, RankedResponse, RankedSongResponse,
};

impl From<&MusicComparison> for MusicComparisonResponse {
    fn from(music: &MusicComparison) -> Self {
        Self {
            a: music.a().into(),
            b: music.b().into(),
            only_in_a: artist_names(music.only_in_a()),
            only_in_b: artist_names(music.only_in_b()),
        }
    }
}

impl From<&MusicSummary> for MusicResponse {
    fn from(summary: &MusicSummary) -> Self {
        Self {
            entries_with_song: summary.entries_with_song,
            artists: summary.artists.iter().map(artist_response).collect(),
            songs: summary.songs.iter().map(song_response).collect(),
        }
    }
}

fn artist_names(artists: &[ArtistName]) -> Vec<String> {
    artists
        .iter()
        .map(|artist| artist.value().to_string())
        .collect()
}

fn artist_response(ranked: &Ranked<ArtistName>) -> RankedResponse {
    RankedResponse {
        name: ranked.item.value().to_string(),
        count: ranked.count,
        share: ranked.share,
    }
}

fn song_response(ranked: &Ranked<(ArtistName, SongTitle)>) -> RankedSongResponse {
    let (artist, title) = &ranked.item;
    RankedSongResponse {
        artist: artist.value().to_string(),
        title: title.value().to_string(),
        count: ranked.count,
        share: ranked.share,
    }
}
