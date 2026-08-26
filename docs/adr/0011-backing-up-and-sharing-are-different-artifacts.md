# Backing up and sharing are two artifacts, not one with options

A CompleteBackup carries everything an account knows. A ShareableExtract carries mood, Content and Activities as a readable markdown document, and nothing else. They are separate endpoints with separate names and separate file extensions.

A single export with checkboxes was rejected. It produces files that look like backups and are not, and the failure mode — someone sharing the wrong file — surfaces only after the disclosure has happened.

## Considered Options

- **A zip of JSON for the extract too**, matching the backup. Rejected because the extract exists to be read by a person: a therapist handed a zip of JSON has been handed nothing. Markdown also makes the distinction visible at the moment of download, where it matters.
- **Restoring through the existing importer.** Rejected for the same reason the split exists. That importer reads foreign formats — Daylio, generic CSV — and its `ImportedRow` knows only mood, date, activities and a note. Teaching it about metrics, cycle starts, reminders and preferences would make every foreign format carry empty fields for them, and would put a Daylio file and a complete backup through one path with very different expectations. Restore is its own endpoint.

## One place to register a dimension

Entry dimensions are written to and read from the backup through `DimensionPayload`, the enum in `api-types` that already maps every `DimensionValue` variant to a wire shape for the HTTP API. Adding a `DimensionValue` variant fails to compile until it is mapped there, and the backup picks it up with no further change. Registration is therefore enforced by the compiler rather than remembered from a document.

This makes an adapter depend on `api-types`, which is unusual here. It is deliberate: a backup file is a wire format, and the alternative was a second exhaustive match over every dimension that could silently fall behind the first.

## Identity across a restore

A backup stores Activity ids alongside their names, and media under their original ids, because entries reference all three by id. A restore mints new ids — media storage assigns them, and an Activity that already exists in the target account keeps the id it has — so the restore builds a translation from old id to new and rewrites each entry's Activities, Photos and VoiceMemos dimensions through it. Without that, every restored entry would reference identifiers that no longer exist and the tags would silently vanish.

An id that cannot be translated is dropped from its dimension rather than restored as a dangling reference.

## What a backup deliberately omits

ApiTokens are absent. They are credentials, not data about days, and restoring one would resurrect a secret the User may have revoked deliberately.

RejectedMetrics are absent. The trace records readings that were never stored; carrying a record of absence into a restore has nothing to restore.

## Restore adds, it never replaces

A restore writes into the account as it stands. This is the honest behaviour for the merge the code performs, and it means restoring twice duplicates entries — the importer's `(logged_at, mood)` dedup does not apply on this path. The alternative, clearing the account first, is a destructive act hidden inside an action a User reaches for when something has already gone wrong.
