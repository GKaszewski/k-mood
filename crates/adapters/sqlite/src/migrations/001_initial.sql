CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY NOT NULL,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    display_name TEXT,
    timezone TEXT,
    role TEXT NOT NULL DEFAULT 'User',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS activities (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL REFERENCES users(id),
    name TEXT NOT NULL,
    category TEXT,
    archived INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS mood_entries (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL REFERENCES users(id),
    mood INTEGER NOT NULL,
    logged_at TEXT NOT NULL,
    content TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS entry_activities (
    entry_id TEXT NOT NULL REFERENCES mood_entries(id) ON DELETE CASCADE,
    activity_id TEXT NOT NULL REFERENCES activities(id),
    PRIMARY KEY (entry_id, activity_id)
);

CREATE TABLE IF NOT EXISTS entry_photos (
    entry_id TEXT NOT NULL REFERENCES mood_entries(id) ON DELETE CASCADE,
    photo_id TEXT NOT NULL,
    PRIMARY KEY (entry_id, photo_id)
);

CREATE TABLE IF NOT EXISTS entry_voice_memos (
    entry_id TEXT NOT NULL REFERENCES mood_entries(id) ON DELETE CASCADE,
    voice_memo_id TEXT NOT NULL,
    PRIMARY KEY (entry_id, voice_memo_id)
);

CREATE TABLE IF NOT EXISTS reminders (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL REFERENCES users(id),
    monday TEXT,
    tuesday TEXT,
    wednesday TEXT,
    thursday TEXT,
    friday TEXT,
    saturday TEXT,
    sunday TEXT,
    enabled INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS refresh_sessions (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL REFERENCES users(id),
    token TEXT NOT NULL UNIQUE,
    expires_at TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_activities_user_id ON activities(user_id);
CREATE INDEX IF NOT EXISTS idx_mood_entries_user_id ON mood_entries(user_id);
CREATE INDEX IF NOT EXISTS idx_mood_entries_logged_at ON mood_entries(logged_at);
CREATE INDEX IF NOT EXISTS idx_reminders_user_id ON reminders(user_id);
CREATE INDEX IF NOT EXISTS idx_refresh_sessions_token ON refresh_sessions(token);
CREATE INDEX IF NOT EXISTS idx_refresh_sessions_user_id ON refresh_sessions(user_id);
