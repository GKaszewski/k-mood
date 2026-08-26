CREATE TABLE IF NOT EXISTS cycle_starts (
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    date TEXT NOT NULL,
    PRIMARY KEY (user_id, date)
);

CREATE TABLE IF NOT EXISTS user_preferences (
    user_id TEXT PRIMARY KEY NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    tracks_cycle INTEGER NOT NULL DEFAULT 0
);
