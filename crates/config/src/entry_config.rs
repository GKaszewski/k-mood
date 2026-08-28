const DEFAULT_MAX_CONTENT_LENGTH: usize = 65_536;
const DEFAULT_MAX_PHOTOS: usize = 10;
const DEFAULT_MAX_VOICE_MEMOS: usize = 5;
const DEFAULT_MAX_ACTIVITIES_PER_ENTRY: usize = 50;
const DEFAULT_MAX_ENTRIES_PER_PAGE: i64 = 200;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(default)]
pub struct EntryConfig {
    pub max_content_length: usize,
    pub max_photos: usize,
    pub max_voice_memos: usize,
    pub max_activities_per_entry: usize,
    pub max_entries_per_page: i64,
}

impl Default for EntryConfig {
    fn default() -> Self {
        Self {
            max_content_length: DEFAULT_MAX_CONTENT_LENGTH,
            max_photos: DEFAULT_MAX_PHOTOS,
            max_voice_memos: DEFAULT_MAX_VOICE_MEMOS,
            max_activities_per_entry: DEFAULT_MAX_ACTIVITIES_PER_ENTRY,
            max_entries_per_page: DEFAULT_MAX_ENTRIES_PER_PAGE,
        }
    }
}
