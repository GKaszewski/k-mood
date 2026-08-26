# Weather is observed after the entry is saved, in the domain's own vocabulary

Weather is resolved server-side from an entry's Location, after the entry is written, and stored as a Condition drawn from a closed vocabulary plus a temperature in celsius. It is always attributed to the Provider that observed it.

Logging a mood is the most latency-sensitive thing the app does, so it cannot wait on a third party. The lookup is an `ObserveWeather` job, swept from "entries with a Location and no Weather" — which satisfies ADR 0004's rule, so losing a queued job costs promptness and never data.

## Considered Options

- **Client-supplied weather.** Three clients would each pick a provider, a vocabulary and a unit, and correlating across inconsistent labels is meaningless. Resolving on the server gives one implementation and one vocabulary.
- **OpenWeatherMap.** Better known, but historical data is a paid tier, so the sweeper could only enrich recent entries — and every self-hoster would need to register a key before weather worked at all. Open-Meteo needs no key, serves history by coordinate and date, and publishes WMO condition codes as a documented integer scale, so the mapping to our vocabulary is a table rather than a guess.
- **Storing the provider's own description.** Rejected: "light intensity shower rain" is not comparable with anything.

## What the vocabulary carries

Condition and temperature. Both plausibly move mood, both are comparable across any two entries, and both correlate — condition as a category, temperature as a continuous input in the Measurements Family. Daylight was considered and left out: it is derivable from coordinates and time with no provider at all, and it is a fact about the calendar and latitude rather than about weather, so it belongs beside MoonPhase if it is wanted.

The vocabulary is deliberately coarse. Open-Meteo distinguishes slight, moderate and dense drizzle; all three are `drizzle` here. A finer vocabulary would multiply the categories without adding signal, and every Provider draws those lines differently.

## Weather cannot be edited away

Every other dimension repository deletes its row when a save arrives without that dimension — that is how a User clears a note or removes a photo. Weather does not. The worker writes it after the fact, and no client ever sends it back, so honouring absence as deletion would mean any edit silently erased it. Absence from a user-supplied dimension list means "not mentioned", and for an observed dimension that is not a request to delete.

## The switch is a switch, not a filter

`worker.look_up_weather = false` means no coordinates leave the machine. It is enforced by not constructing the lookup at all, so there is nothing that could make a request. It also stops the weather sweep entirely: an early version left the sweep running, which enqueued jobs that failed on every attempt until they exhausted — churn and noise for someone who deliberately turned the feature off. If weather cannot be observed there is no weather backlog. Turning it back on catches up the whole backlog on the next sweep.

## What is not built

The issue asks for backlogged lookups to be grouped by rounded coordinates and date range so a backlog collapses into few requests. It is not implemented: each job resolves one entry with one request. Open-Meteo accepts a date range for one coordinate, so the grouping is possible, but it needs a batch shape the job queue does not have — one job would have to stand for many entries, which breaks the one-job-one-subject rule the queue and its sweep are built on. The realistic backlog is small, as this ADR's own reasoning notes: imported history carries no coordinates, so nothing but an outage or an offline client produces one. The sweep's configured bound is the guard until that stops being true.
