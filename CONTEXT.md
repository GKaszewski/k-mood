# k-mood

A personal mood tracking journal. Users log how they feel throughout the day, tag entries with activities, and review trends over time. Multi-user, self-hosted, single server with multiple clients.

## Language

### Core

**MoodEntry**:
A single mood record — the aggregate root. Every MoodEntry has exactly one Mood and belongs to exactly one User. May optionally include Activities, Content, photos, and voice memos. Multiple MoodEntries per day are allowed.
_Avoid_: Log, journal entry, record, mood log

**Mood**:
One of five discrete states representing how the user feels, mapped to a 1–5 ordinal scale: Awful (1), Bad (2), Meh (3), Good (4), Rad (5). The ordering is a domain truth — Rad is better than Good. Required on every MoodEntry.
_Avoid_: Feeling, emotion, state, score

**Content**:
Optional markdown text attached to a MoodEntry. Serves both quick annotations ("went for a walk") and longer journal-style writing. No domain-level length limit — the application layer enforces configurable maximums.
_Avoid_: Note, quick note, description, journal entry

**Activity**:
A named item from a user's personal catalog, tagged onto MoodEntries. Covers heterogeneous concepts (social contexts like "friends", health actions like "exercise", sleep indicators like "good sleep") under one umbrella term. Each Activity belongs to exactly one User. Activities are archivable — archived Activities remain on historical entries but cannot be tagged onto new ones.
_Avoid_: Tag, label, habit, tracker

**Category**:
A display-only grouping label stored as an optional string on an Activity (e.g., "social", "health", "sleep"). The domain stores it but never interprets it — no domain logic references categories. Exists purely for UI organization across multiple clients.
_Avoid_: Group, section, type

### People

**User**:
A registered account identified by username and email. Has a role (Admin or User) and an optional timezone for analytics display. Owns their own Activity catalog, MoodEntries, and Reminders.
_Avoid_: Account, member, profile

### Scheduling

**Reminder**:
A per-user notification schedule. Each Reminder defines an `Option<Time>` for each day of the week — `Some(20:00)` means remind at 8 PM, `None` means skip that day. Can be enabled or disabled. A User can have multiple Reminders (e.g., one for morning check-in, one for evening). The domain defines when to remind; clients decide the message and delivery mechanism.
_Avoid_: Notification, alert, alarm, push
