CREATE TABLE IF NOT EXISTS provider_connections (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider TEXT NOT NULL,
    credential BLOB NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE (user_id, provider)
);

CREATE INDEX IF NOT EXISTS idx_provider_connections_user_id ON provider_connections(user_id);
