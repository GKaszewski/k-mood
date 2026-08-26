CREATE TABLE IF NOT EXISTS entry_location (
    entry_id TEXT PRIMARY KEY NOT NULL REFERENCES mood_entries(id) ON DELETE CASCADE,
    latitude REAL NOT NULL,
    longitude REAL NOT NULL
);

CREATE TABLE IF NOT EXISTS entry_song (
    entry_id TEXT PRIMARY KEY NOT NULL REFERENCES mood_entries(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    artist TEXT NOT NULL,
    album TEXT,
    recording_id TEXT
);
