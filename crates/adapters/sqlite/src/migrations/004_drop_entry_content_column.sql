INSERT OR IGNORE INTO entry_content (entry_id, content)
SELECT id, content FROM mood_entries WHERE content IS NOT NULL;

ALTER TABLE mood_entries DROP COLUMN content;
