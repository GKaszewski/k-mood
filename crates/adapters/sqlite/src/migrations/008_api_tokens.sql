CREATE TABLE IF NOT EXISTS api_tokens (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    digest TEXT NOT NULL UNIQUE,
    scope TEXT NOT NULL,
    created_at TEXT NOT NULL,
    last_used_at TEXT,
    UNIQUE (user_id, name)
);

CREATE INDEX IF NOT EXISTS idx_api_tokens_user_id ON api_tokens(user_id);
