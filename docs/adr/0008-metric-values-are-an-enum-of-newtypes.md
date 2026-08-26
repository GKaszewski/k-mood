# A metric's kind and value are one type, not two fields

`MetricValue` is an enum whose every variant wraps its own validated newtype — `Steps(Steps)`, `SleepMinutes(SleepMinutes)`, and so on. `MetricKind` remains as a payload-free discriminant for naming a kind without a value, but it is derived from the value and never stored alongside it.

The obvious shape — a struct holding a kind and a loose numeric value — makes invalid states constructible: a step count tagged as HRV compiles cleanly, because the kind is data and the value is untyped. Collapsing them removes that state from the language rather than from code review.

## Considered Options

- **Trait objects (`Box<dyn Metric>`)** — the more open shape, and consistent with this codebase's use of `dyn` for ports. Rejected because it gives up exhaustiveness: a new metric would compile while correlation and serialization silently failed to handle it, which is precisely the class of error the type system was brought in to catch. Persistence would need a parse-by-kind registry regardless, so the enumeration reappears at the boundary anyway.
- **`DailyMetric<M: Metric>`** — strictest typing, but mixed metrics cannot share a collection, which defeats iterating every kind to correlate it.

## Consequences

Each newtype validates its own range in `new()`, following the established value-object pattern, and units live in the type rather than in a field name — `Hrv` *is* milliseconds. Eleven near-identical bounded-integer newtypes justify a macro, as `uuid_id!` already does for identity types.

Hydration is fallible in a way entry loading is not: a row can carry a kind this build does not know, or a value a later-tightened range now rejects. Such rows are skipped and recorded in the same rejection trace that receives invalid imports, rather than failing the query.

Correlation needs a numeric projection that necessarily discards the type again. That is confined to one place.

One MetricKind carries exactly one unit. This is what forced caffeine out of the set: providers report milligrams and users report cups, and nothing converts between them honestly.

## The rejection trace, once it existed

The hydration failure this document describes now writes to the same trace that receives invalid imports, as promised. The metric query repository holds a `RejectionCommandPort` and records what it skipped: a read with a side effect, which is unusual enough to name. The alternative was returning the unreadable rows alongside the readable ones and letting a caller decide, which pushes a decision no caller has an opinion about into every one of them. A failure to write the trace is logged and swallowed — a reading that cannot be recorded as unusable must still not fail the query that found it.
