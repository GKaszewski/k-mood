#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum DimensionKind {
    Content,
    Activities,
    Photos,
    VoiceMemos,
    Location,
    Song,
    Weather,
}
