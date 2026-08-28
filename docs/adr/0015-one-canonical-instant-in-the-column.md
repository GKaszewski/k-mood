# A MoodEntry's instant is stored in one canonical form

`logged_at` is a `DateTime<FixedOffset>` written to a TEXT column with `to_rfc3339()`, so the offset it happened to arrive in went into the column. Every range query compares that column as a string:

```sql
WHERE user_id = ? AND logged_at >= ? AND logged_at <= ? ORDER BY logged_at DESC
```

Text ordering only agrees with instant ordering while every row carries the same offset. `2026-08-25T00:30:00+02:00` sorts after `2026-08-25T00:00:00+00:00` and precedes it in time. Mixed offsets are reachable: `update_profile` lets an account change timezone, and the importer stamps each row with the offset in force on that row's own date, so a year of imported history already spans two.

The in-memory fake could not catch this. It compares `DateTime<FixedOffset>` values, and chrono compares instants — the fake was right and the database was wrong, which is the one direction the fake rules do not warn about.

## Considered Options

- **A second sortable column beside `logged_at`** — keeps the offset a row arrived in. Two representations of one fact, both writable, and nothing forces them to agree.
- **An integer epoch column** — sorts and compares correctly and reads as nothing at all in a `sqlite3` session, on a table that is otherwise legible text.

## Consequences

`sortable_instant` converts to UTC and formats to second precision, and is the single place any instant becomes column text. Writes, range predicates and the cascade's range delete all go through it, so a query cannot be written against one convention and stored data another.

The offset a client sent is not preserved. Per ADR 0001 nothing derives a day boundary from it — the User's timezone does that — so the offset was a rendering, not data. `MoodEntry` carries the instant it was logged, and one instant now has one spelling. The SPA reads `loggedAt` through `new Date(...)` in every place it touches it, so it renders in the viewer's own zone either way.

Second precision is the granularity the importer works in, and it makes two spellings of one moment compare equal.

Migration 015 rewrites existing rows with SQLite's own time parser, which reads the offset suffix and normalizes to UTC, and adds an index on `(user_id, logged_at)` now that the column's order means something.

## Import dedup had to move with it

`import_entries` skipped a row when `(logged_at.to_rfc3339(), mood)` matched a stored entry — a comparison of renderings. Once stored rows read back as `+00:00` and the importer derives `+02:00` for the same instant, that key stops matching and a re-import duplicates the whole file.

`AlreadyHere` keys on the instant and the mood instead, so the same moment is the same entry whichever offset either side is spelled in. It is also updated as rows are accepted, which the old set was not, so a file that repeats a row internally no longer imports it twice.
