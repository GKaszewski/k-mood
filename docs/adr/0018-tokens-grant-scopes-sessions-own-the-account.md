# A token grants named scopes; only a session owns the account

`ApiToken` carried a single-variant `TokenScope::WriteMetrics`, and only the metric-write routes accepted one. Every other route took `AuthenticatedUser`, which validated a JWT and nothing else. A client that was not the SPA therefore had exactly two options: write metrics, or hold the account's password and log in as the user.

That is the right boundary for an importer and the wrong one for a desktop widget, which needs to read entries and write them and should never be trusted with the password.

## Considered Options

- **Let `AuthenticatedUser` accept any api token** — one line, and it hands every token the whole account, including minting further tokens and restoring a backup over the top of the data. A credential that can mint credentials is not a narrower credential.
- **One scope per token** — matches the column that already existed and forces a widget to juggle two secrets to read and write. The set is the natural unit; a token is a role, not a permission.

## Consequences

`TokenScopes` is a non-empty set, so a token that grants nothing cannot be minted, and an unknown scope name is refused rather than dropped — a typo must not quietly produce a narrower token than the one asked for. A stored set containing a scope this build cannot read makes the whole token unreadable, the same way an unreadable metric row is refused rather than half-understood.

The four scopes name what a client does rather than which routes it calls: `readJournal`, `writeJournal`, `writeMetrics`, `readProfile`. Routes are grouped behind one extractor each — `JournalReader`, `JournalWriter`, `MetricWriter`, `ProfileReader` — so the scope a route needs is visible in its signature and a new route must choose one to compile.

`SessionUser` replaces `AuthenticatedUser` and accepts a JWT only. It guards what no token may reach: the password, the profile, provider credentials, push registration, backup, restore, import, and minting or revoking tokens. The rename is the point — "authenticated" no longer distinguishes the two things that can authenticate.

`writeMetrics` keeps its old meaning exactly, so the token's name still becomes the Provider its metric writes are attributed to. A token without `writeMetrics` never reaches that path and its name is only a label.

Migration 016 backfills every existing token with `writeMetrics`, which is what it already had, and drops the old column. An importer minted before this change keeps working and gains nothing.
