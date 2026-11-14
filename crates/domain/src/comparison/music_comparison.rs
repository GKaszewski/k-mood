use std::cmp::Reverse;

use crate::song::ArtistName;

use super::MusicSummary;

#[derive(Debug, Clone, PartialEq)]
pub struct MusicComparison {
    a: MusicSummary,
    b: MusicSummary,
    only_in_a: Vec<ArtistName>,
    only_in_b: Vec<ArtistName>,
}

impl MusicComparison {
    pub fn between(a: MusicSummary, b: MusicSummary) -> Self {
        let only_in_a = most_played_missing_from(&a, &b);
        let only_in_b = most_played_missing_from(&b, &a);
        Self {
            a,
            b,
            only_in_a,
            only_in_b,
        }
    }
}

impl MusicComparison {
    pub fn a(&self) -> &MusicSummary {
        &self.a
    }

    pub fn b(&self) -> &MusicSummary {
        &self.b
    }

    pub fn only_in_a(&self) -> &[ArtistName] {
        &self.only_in_a
    }

    pub fn only_in_b(&self) -> &[ArtistName] {
        &self.only_in_b
    }
}

fn most_played_missing_from(present: &MusicSummary, absent: &MusicSummary) -> Vec<ArtistName> {
    let others = absent.all_artists();
    let mut missing: Vec<&ArtistName> = present
        .all_artists()
        .into_iter()
        .filter(|artist| !others.contains(artist))
        .collect();
    missing.sort_by_key(|artist| Reverse(present.plays_of(artist)));
    missing.truncate(present.top());
    missing.into_iter().cloned().collect()
}
