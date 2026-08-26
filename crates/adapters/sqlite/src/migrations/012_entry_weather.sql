CREATE TABLE IF NOT EXISTS entry_weather (
    entry_id TEXT PRIMARY KEY NOT NULL REFERENCES mood_entries(id) ON DELETE CASCADE,
    condition TEXT NOT NULL,
    temperature REAL NOT NULL,
    observed_by TEXT NOT NULL
);
