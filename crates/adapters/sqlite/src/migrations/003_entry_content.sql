CREATE TABLE IF NOT EXISTS entry_content (
    entry_id TEXT PRIMARY KEY NOT NULL REFERENCES mood_entries(id) ON DELETE CASCADE,
    content TEXT NOT NULL
);
