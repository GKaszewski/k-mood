CREATE TABLE IF NOT EXISTS metric_rejections (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    origin TEXT NOT NULL,
    provider TEXT,
    date TEXT,
    kind TEXT NOT NULL,
    value INTEGER,
    reason TEXT NOT NULL,
    recorded_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_metric_rejections_user ON metric_rejections(user_id, recorded_at);
