# Background work runs in a separate binary against a deliberately lossy queue

Background jobs (weather enrichment, metadata lookup, reminders) run in a dedicated worker binary, not in `tokio::spawn` calls inside the server. Correctness comes from sweeping domain state; the queue exists only for promptness and observability.

The rule that makes this safe: **no job may be enqueued unless a query over domain state can independently rediscover the same work.** "Entries with a Location and no Weather" is a valid job; anything with no trace in the data is not.

## Considered Options

- **Transactional outbox** — events written atomically with the change that caused them. Correct, but `MoodEntryCommandPort::save` and `EventPublisherPort::publish` are separate ports with no shared transaction, so this required either an aggregate-recorded event collection or a unit-of-work threaded through every command. Both are real cost for a guarantee that sweeps already provide.
- **Polling only, no queue** — matches how reminders already work, but gives no record of a failed enrichment, so nothing can be retried deliberately or inspected.

## Consequences

Saving a MoodEntry stays a single small transaction that must simply succeed. Everything downstream is best-effort with a visible trace — status, attempt count, last error — so a failure can be retried rather than disappearing.

Losing a queued job costs latency, never data. This means the queue needs no transactional guarantees, and restarts, crashes, and imports that predate the queue are all handled by the same sweep rather than by three separate mechanisms.

The cost is that every job type owes a sweeper query alongside its enqueue, and jobs that cannot be expressed as a query over state must be refused rather than accommodated.

The in-process `mpsc` event channel cannot cross a process boundary, so it is not the transport. The queue's backing store is an adapter choice behind the port — SQLite is the store of record; NATS, where available, is a relay for fan-out, never the source of truth.

`create_pool` must set a `busy_timeout`: two processes writing one SQLite file without it produces intermittent `SQLITE_BUSY` failures. Shared wiring must move out of `crates/server` so both binaries build the same object graph.

## Bounds

Sweeps are rate-bounded from configuration, not from constants. The realistic backlog is small — the Daylio importer carries no location data, so imported history never triggers weather lookups; the sweeper exists for entries stranded by an upstream outage or an offline client. The bound is a safety limit rather than a throughput design.

Where an upstream accepts a date range for one location, a backlog grouped by rounded coordinates collapses into a handful of requests rather than one per entry.

Now-playing lookup is not a job. It resolves synchronously while an entry is being composed, and a MoodEntry never acquires a Song afterwards — so there is nothing for a sweep to rediscover, and nothing belongs on the queue.

## Correcting this document on SQLITE_BUSY

This ADR said `create_pool` sets no `busy_timeout` and that two processes writing one file would therefore fail intermittently. Both halves were wrong for the sqlx version in use.

sqlx 0.9 already applies a five-second `busy_timeout` by default, so the setting was never absent. It is now set explicitly at ten seconds, because a value this operationally significant should be visible in the code rather than inherited from a dependency's default — but raising it fixed nothing, and removing it entirely breaks no test.

The real failure was different and `busy_timeout` cannot help with it. Claiming jobs was written as a SELECT followed by UPDATEs inside a deferred transaction. In WAL mode, upgrading a transaction from reading to writing fails with `SQLITE_BUSY_SNAPSHOT` (code 517) **immediately**, not after the timeout: the reader is holding a snapshot that a concurrent commit has already superseded, and no amount of waiting can reconcile it. Two writers on one file failed within milliseconds.

The fix is to claim in one statement — `UPDATE ... WHERE id IN (SELECT ...) RETURNING ...` — which takes the write lock up front and never upgrades. That is also simpler than the transaction it replaced. Any future queue operation that reads and then writes must either be a single statement or open its transaction as `BEGIN IMMEDIATE`.

## A job stored in a shape this build cannot read

Rows are hydrated defensively, as metrics are. A row whose kind, status or timestamps cannot be read is marked exhausted rather than skipped: skipping leaves it claimable, so the claim would flip it to running, fail to read it, and the stall reclaim would hand it back — a silent loop with no progress and no trace. Marking it exhausted stops the churn and leaves it visible, which is what the queue is for.

## What the worker owns

Reminder scheduling and expired-session cleanup moved out of `tokio::spawn` calls in the server and into the worker, alongside the queue loop and the sweep. The server logs that background work lives elsewhere, because a deployment that runs only the server now silently does no background work at all — `docker-compose` gains a second service, and the image carries both binaries.

Shared wiring moved to `crates/bootstrap`, which both binaries depend on, so there is one object graph rather than two that can drift.

## Two processes starting at the same moment

Both binaries run migrations on startup through the shared factory, so `docker compose up` — or a dev target that launches both — starts two processes racing to migrate one file. Each saw a migration as unapplied, each applied it, and the second `INSERT INTO schema_migrations` failed with `UNIQUE constraint failed`, killing whichever process lost. `depends_on` does not help: it waits for a container to start, not to finish migrating.

The whole migration run now happens inside one `BEGIN IMMEDIATE` transaction, so the second process waits on `busy_timeout` and then finds everything already applied. `BEGIN IMMEDIATE` rather than a plain `BEGIN` because the run reads `schema_migrations` before writing to it, and this document already records what a deferred read-to-write upgrade does in WAL mode.

Making the run safe was preferred to electing one process as the migrator. Coordination would need a startup order the compose file cannot guarantee, and a worker that must not touch the schema is a rule nothing enforces.

A test starts two pools on one file and migrates concurrently. It reliably fails with no transaction at all; it does **not** distinguish `IMMEDIATE` from deferred, because the timing in practice has one run complete before the other upgrades. The choice of `IMMEDIATE` rests on the failure mode proven elsewhere in this document rather than on that test.
