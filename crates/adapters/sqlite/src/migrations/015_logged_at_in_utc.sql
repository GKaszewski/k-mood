UPDATE mood_entries
SET logged_at = strftime('%Y-%m-%dT%H:%M:%S+00:00', logged_at)
WHERE logged_at NOT LIKE '%+00:00';

CREATE INDEX IF NOT EXISTS idx_mood_entries_user_logged_at
    ON mood_entries(user_id, logged_at);
