CREATE TABLE IF NOT EXISTS daily_metrics (
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    date TEXT NOT NULL,
    kind TEXT NOT NULL,
    value INTEGER NOT NULL,
    provider TEXT,
    PRIMARY KEY (user_id, date, kind)
);
