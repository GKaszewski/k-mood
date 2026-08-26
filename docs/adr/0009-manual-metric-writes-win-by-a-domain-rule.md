# A manual DailyMetric write wins by a domain rule the store obeys

`DailyMetric::supersedes` decides whether an incoming metric replaces the one already stored for a `(User, Date, MetricKind)`. A metric sourced `Manual` always supersedes; a metric sourced from a Provider supersedes anything except a `Manual` one. The SQLite store reads the existing row inside a transaction, asks the domain, and writes only on a true answer.

The precedence rule is domain knowledge, so it is stated once, in the domain, where it can be tested without a database.

## Considered Options

- **A conditional upsert in SQL** — `ON CONFLICT (user_id, date, kind) DO UPDATE ... WHERE excluded.source = 'manual' OR daily_metrics.source != 'manual'`. One atomic statement and no read. Rejected because the rule then lives in a string the domain tests cannot reach, and the in-memory store used by every application test has to reimplement it by hand — two expressions of one piece of knowledge, free to disagree. The transaction the chosen option needs is a smaller cost than that.
- **The use case compares before writing** — testable without a database, but the check spans two port calls with no transaction around them, and every future write path is free to forget it. Import (#13) is a second write path, which makes that a matter of when rather than whether.

## Consequences

`DailyMetricCommandPort::save` takes a batch and is the only way a metric reaches storage. Both the SQLite store and `InMemoryDailyMetricStore` call `supersedes`, so a fake that has drifted from the real store fails its own tests.

A refused write is not an error. A Provider importing a day the User has already stated by hand succeeds and changes nothing, because the alternative — failing the import — makes one hand-entered day poison a year of backfill.

## Hydration rejections

A stored row can carry a kind this build does not know, or a value a range tightened since it was written now rejects. Such a row is skipped and logged with its user, date and kind; the surrounding rows are returned. ADR 0008 places these in the same rejection trace that receives invalid imports, and that trace arrives with the import endpoint (#13), which is the consumer that gives it its shape. Until then the record is a log line, and the call site that emits it is the one #13 redirects.

## On the wire

A DailyMetric is read as `{ date, kind, value, provider }`, where an absent `provider` means the User stated the value. One nullable field carries the whole of Source, which keeps the wire in step with the column and leaves no way to describe a metric that is manual and from a Provider at once. A tagged `source` object was the alternative; it spends a nested shape on a distinction one field already makes, and a Provider may legitimately be named `manual`, which rules out flattening the two into a single string.

Writes never carry a source. Every metric arriving over HTTP is `Manual` by construction, because a Provider does not use this endpoint — the import path (#13) builds its own metrics and is the only thing that can produce a Provider source.

## Clearing

A metric is cleared by stating no value for its kind — `{"kind": "steps", "value": null}` on the same `PUT`. There is no delete endpoint: one write path into `daily_metrics` means clearing cannot disagree with writing about what a day holds, and the day sheet gets it for nothing, since emptying a field is already how a person says a reading should not be there.

Clearing removes the row, and a later import for that day is then free to report the kind again. The manual-wins rule therefore covers stated values but not absence: the only way to keep a Provider's reading out for good is to state one you believe.

A tombstone — a cleared kind that stays cleared until the User states a value — was the alternative, and it would close that gap. It was rejected because it needs a DailyMetric that exists with no value, which the domain deliberately cannot express: `MetricValue` is not optional, and making it optional to record an absence would put `None` into every match over every kind, for one case. The gap it leaves is small in practice, because a Provider re-reporting what it already reported is the store telling the truth about what that Provider says.

A request naming the same kind twice is refused rather than resolved by order, because "state 8412 steps and also clear steps" has no reading that is obviously right.
