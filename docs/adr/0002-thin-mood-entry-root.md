# MoodEntry is a thin root; everything optional is an independent EntryDimension

`MoodEntry` carries only its identity, one Mood, and the instant it was logged. Every optional aspect — Content, Activities, photos, voice memos, weather, location, song — is an EntryDimension owning its own type, table, port, and validation, and is composed onto the entry at read time.

A dimension is a new self-contained module owning its own storage and validation. Adding one does not change any existing dimension, repository, or use case.

It does, however, add a variant to a closed `DimensionValue` enum, mirroring `MetricValue` in ADR 0008. This is deliberate: the compiler then walks you through every site that must handle the new dimension — composer, wire dispatch, export — rather than letting it silently fail to appear. Openness was traded for exhaustiveness, on the same reasoning and for the same reason as ADR 0008.

## Considered Options

- **Adding nullable fields to `MoodEntry`** — what the codebase did. The aggregate was already at 10 fields against a stated 5-6 guideline, and the proposed data points would have taken it past 30.
- **A closed facet enum on the root** — keeps the root small but forces heterogeneous payloads into one table as JSON or sparse columns, and turns "at most one weather per entry" from structure into a runtime check.

## Consequences

The existing four dimensions were retrofitted rather than left as a parallel path, so there is exactly one answer to "what is attached to this entry". Reads batch-load per dimension across a whole page of entries rather than per entry.

Enumeration does not disappear — it concentrates in the read composer and the wire format's kind dispatch, instead of being spread across the domain.
