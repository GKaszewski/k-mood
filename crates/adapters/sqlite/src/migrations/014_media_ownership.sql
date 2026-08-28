CREATE TABLE IF NOT EXISTS media_owners (
    kind TEXT NOT NULL,
    media_id TEXT NOT NULL,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TEXT NOT NULL,
    PRIMARY KEY (kind, media_id)
);

CREATE INDEX IF NOT EXISTS idx_media_owners_user_id ON media_owners(user_id);

INSERT OR IGNORE INTO media_owners (kind, media_id, user_id, created_at)
SELECT 'photo', ep.photo_id, me.user_id, me.created_at
FROM entry_photos ep
JOIN mood_entries me ON me.id = ep.entry_id;

INSERT OR IGNORE INTO media_owners (kind, media_id, user_id, created_at)
SELECT 'voice_memo', evm.voice_memo_id, me.user_id, me.created_at
FROM entry_voice_memos evm
JOIN mood_entries me ON me.id = evm.entry_id;
