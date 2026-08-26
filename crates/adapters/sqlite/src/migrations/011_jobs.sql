CREATE TABLE IF NOT EXISTS jobs (
    id TEXT PRIMARY KEY NOT NULL,
    kind TEXT NOT NULL,
    subject TEXT NOT NULL,
    status TEXT NOT NULL,
    attempts INTEGER NOT NULL DEFAULT 0,
    last_error TEXT,
    enqueued_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE (kind, subject)
);

CREATE INDEX IF NOT EXISTS idx_jobs_claimable ON jobs(kind, status, enqueued_at);
