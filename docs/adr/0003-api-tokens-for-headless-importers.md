# Headless importers authenticate with long-lived scoped API tokens

Health data arrives from automations — iOS Shortcuts, Tasker, cron — that cannot participate in the session flow. Users mint named, revocable, non-expiring API tokens scoped to writing DailyMetrics only.

## Considered Options

- **Reusing the refresh token flow** — rejected because refresh tokens rotate and revoke the old token on use, so a single interrupted run locks the importer out permanently and silently, and iOS Shortcuts has no durable place to keep a rotating secret.
- **Raising the access token TTL** — rejected because it would lengthen the compromise window for every browser session to solve an automation problem.

## Consequences

The system now has two credential types with different lifetimes and different powers. An API token cannot read entries or change account settings, so a leaked one exposes less than a session token — but it does not expire, so listing and revoking tokens is a required part of the settings surface, not a nice-to-have.

## What a token is, in practice

The value is `kmood_` followed by 32 random bytes in url-safe base64 — the prefix so a secret scanner can recognise one in a repository, the bytes because 256 bits needs no stretching. Only its SHA-256 digest is stored, in a unique indexed column, so authentication is one indexed lookup and one fast hash. Argon2 would be the wrong tool twice over: it exists to slow down guessing at low-entropy human passwords, and because its output is salted per row it cannot be looked up, which would mean verifying a presented secret against every stored token on every request.

Scope is enforced by which extractor a route asks for, not by a check inside one. `AuthenticatedUser` accepts sessions only; `MetricWriter` accepts either a session or a token. A new endpoint therefore refuses tokens by default, and making one token-accessible is a visible, deliberate edit. The inverse — one extractor consulting a scope table — would make omission the permissive case.

## The name is the Provider

A token's name is a ProviderName, and its writes are attributed to it. This keeps ADR 0009's manual-wins rule intact once automations exist: an import genuinely is a Provider write, so it cannot overwrite what the User stated by hand, and a User correcting an imported day supersedes the importer. The constraint it imposes is that token names live in ProviderName's alphabet — lowercase letters, digits and hyphens — which is surfaced as a validation error rather than silently normalised.

One asymmetry follows from the same rule and is enforced separately: a Provider-attributed request may not clear a metric. Clearing is how a User says a reading should not be there, and an importer that could clear would be able to delete hand-entered data it is not allowed to overwrite.

Tokens survive clearing account data, as ProviderConnections do — a credential is not data about days. Deleting the account removes them through the foreign key.
