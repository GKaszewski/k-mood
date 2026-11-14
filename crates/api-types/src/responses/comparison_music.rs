#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RankedResponse {
    pub name: String,
    pub count: usize,
    pub share: f64,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RankedSongResponse {
    pub artist: String,
    pub title: String,
    pub count: usize,
    pub share: f64,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MusicResponse {
    pub entries_with_song: usize,
    pub artists: Vec<RankedResponse>,
    pub songs: Vec<RankedSongResponse>,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MusicComparisonResponse {
    pub a: MusicResponse,
    pub b: MusicResponse,
    pub only_in_a: Vec<String>,
    pub only_in_b: Vec<String>,
}
