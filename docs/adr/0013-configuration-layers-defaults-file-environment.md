# Configuration arrives in three layers: defaults, then the file, then the environment

Every setting has a compiled-in default. `config.toml` overrides the defaults it names. Environment variables prefixed `KMOOD_` override both. The layering is one expression in `crates/config/src/loader.rs`, so a new setting is a new struct field and nothing else.

A single underscore stays inside a field name, a double underscore descends a section: `KMOOD_AUTH__JWT_SECRET` reaches `auth.jwt_secret`, and `KMOOD_SERVER__CORS__ALLOW_ANY_ORIGIN` reaches two levels down. Splitting on a single underscore would have been ambiguous the moment a field was called `data_dir`.

## Why the environment had to reach everything

CODE_STYLE has always said secrets live in environment variables and tunables live in the file. Until this change nothing in the workspace read an environment variable at all, so `auth.jwt_secret`, `push.vapid_private_key`, `provider.encryption_key` and the S3 keys were only settable by writing them into a file next to the binary — the arrangement the rule exists to prevent.

Stopping at secrets would have honoured the letter of that rule and left the useful half undone. A container that cannot be told its own port or data directory needs a bind-mounted file to differ from any other container running the same image, which puts deployment shape back into a committed artifact. The rule is a floor: secrets *must* be settable from the environment. It is not a ceiling.

## Considered Options

- **A hand-rolled overlay.** Read each variable in `bootstrap` and assign the field. No new dependency and completely transparent, but roughly a hundred and fifty lines that must be edited every time a setting is added — a second place to update for one logical change, which is the thing "single call, cascading changes" exists to forbid. Rejected: the cost is paid forever, by whoever adds the next field.
- **`config-rs`.** Does the same layering, but the crate is named `config` and so is ours. Every import in the crate would need renaming to stay readable. Rejected on the name alone.
- **`figment`.** Layering is its whole purpose, it reads the `serde` derives already on these structs, and it needed no change to any of the ten config types — not even a `Serialize` derive, because container-level `#[serde(default)]` already supplies the base layer. Chosen.

## Why loading moved out of bootstrap

The loader used to live in `bootstrap`, which the testing table in CODE_STYLE marks as untested by design. Precedence between three layers is exactly the logic that needs pinning, and it cannot be pinned where tests are not written. So `load()` now sits in `crates/config` beside the types it populates, and `bootstrap` re-exports it so the binaries are unchanged.

This also puts the defaults and the means of overriding them in one crate. `AppConfig::default()` and the layering that overrides it were previously two crates apart, and only one of them was reachable from a test.

## Refusal is better than a silent default

A malformed `config.toml`, or a variable that cannot be parsed as its field's type, fails startup. figment names the offending key — `invalid type: found string "not-a-port", expected u16 for key "SERVER.PORT"` — which is worth more than a server that comes up on a port nobody chose.

A missing `config.toml` is not an error: the defaults are a complete configuration and running with no file is a supported way to run. But a file *named* by `KMOOD_CONFIG_FILE` and not found is an error, because naming it is a statement that it exists, and silently ignoring the typo would start the server with settings the operator did not choose.

## What is logged

The file that was read, and the *names* of the `KMOOD_` variables that took effect. Never their values: four of the settings reachable this way are secrets, and a log line is the wrong place to learn one.

## Lists are arrays, not delimited strings

`KMOOD_SERVER__CORS__ALLOWED_ORIGINS=["https://a.example","https://b.example"]`. A comma-separated form would read more naturally in a shell, but it would be a second syntax for the same data, parsed differently from the file. One syntax, one parser.
