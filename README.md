# K-Mood

Self-hosted mood tracking journal. Log how you feel, then find out what actually moves it.

## Features

- **Mood tracking** with 5 discrete states (Awful, Bad, Meh, Good, Rad)
- **Activities** with custom categories
- **Rich entries** with markdown notes, photos, voice memos, location, and what you were listening to
- **Daily metrics** — steps, sleep, resting heart rate, HRV, exercise, screen time, alcohol — entered by hand or imported from a phone
- **Correlation** between anything measured and your mood, with several methods run side by side; agreement between them is the signal, not any single number
- **Weather** resolved from an entry's location, and cycle tracking if you want it, both correlated like anything else
- **Trends** including streaks, distribution, and a calendar heatmap
- **Importer tokens** so a Shortcut or a cron script can send metrics without your password
- **Import** from Daylio CSV or any CSV with a mapping wizard
- **Two exports** that cannot be confused: a complete backup for keeping, and a readable journal for sharing
- **Multi-user** with JWT authentication and per-user data isolation
- **PWA** installable on mobile and desktop
- **Self-hosted** with SQLite and local or S3 media storage

## Quick Start

### Docker

```bash
docker run -d --name k-mood \
  -p 3000:3000 \
  -v k-mood-data:/data \
  -e KMOOD_AUTH__JWT_SECRET=your-secret-here \
  ghcr.io/gabrielkaszewski/k-mood:latest

docker run -d --name k-mood-worker \
  -v k-mood-data:/data \
  -e KMOOD_AUTH__JWT_SECRET=your-secret-here \
  --entrypoint k-mood-worker \
  ghcr.io/gabrielkaszewski/k-mood:latest
```

Open `http://localhost:3000`, register an account, and start logging.

The second container is the worker. Everything that happens on a schedule lives there — sending reminders, clearing expired sessions, resolving weather from an entry's location, and filling in the identity of songs logged while an upstream was unavailable. Run the server alone and none of that happens; nothing else breaks.

### Docker Compose

```yaml
services:
  k-mood:
    image: ghcr.io/gabrielkaszewski/k-mood:latest
    ports:
      - "3000:3000"
    volumes:
      - k-mood-data:/data
    environment:
      - KMOOD_AUTH__JWT_SECRET=your-secret-here

  k-mood-worker:
    image: ghcr.io/gabrielkaszewski/k-mood:latest
    entrypoint: ["k-mood-worker"]
    volumes:
      - k-mood-data:/data
    environment:
      - KMOOD_AUTH__JWT_SECRET=your-secret-here
    depends_on:
      - k-mood

volumes:
  k-mood-data:
```

### From Source

Requires Rust 1.88+ (the code uses let-chains, stable since 1.88) and Bun.

```bash
make run
```

That builds the frontend, then starts the server with the worker beside it. Stopping the server stops both. `make run-server` and `make run-worker` start one at a time, `make dev` is the same pair with debug logging, and `make check` runs everything CI would.

## Configuration

Three layers, each overriding the one before it: compiled-in defaults, then `config.toml`, then the environment. Copy `config.example.toml` to `config.toml` and adjust as needed — or set nothing at all and run on the defaults.

| Section | Key | Default | Description |
|---------|-----|---------|-------------|
| `server` | `host` | `0.0.0.0` | Bind address |
| `server` | `port` | `3000` | HTTP port |
| `server.cors` | `allow_any_origin` | `true` | CORS policy |
| `auth` | `jwt_secret` | none | JWT signing key. Required — the server refuses to start without it |
| `auth` | `allow_registration` | `true` | Enable new user registration |
| `storage` | `data_dir` | `./data` | SQLite and media storage path |
| `storage.media` | `backend` | `local` | `local` or `s3` |
| `analysis` | `minimum_sample_size` | `30` | Days needed before any correlation is shown |
| `analysis` | `false_discovery_rate` | `0.10` | How often a marked correlation is expected to be a fluke |
| `import` | `maximum_days_per_import` | `90` | Longest health import accepted in one request |
| `import` | `rejections_kept` | `200` | Unusable readings kept per account, newest first |
| `worker` | `look_up_weather` | `true` | Set `false` and no coordinates leave the machine |
| `worker` | `most_attempts` | `5` | Failures before a job stops retrying and stays visible |
| `worker` | `sweep_seconds` | `900` | How often stranded work is rediscovered |

Every key has a default, so a section you leave out simply uses them. The full set with comments is in `config.example.toml`.

### From the environment

Every key has an environment twin: the prefix `KMOOD_`, a double underscore for each section you descend, and the key spelled as it is in the file.

```bash
KMOOD_SERVER__PORT=8080
KMOOD_AUTH__JWT_SECRET=...
KMOOD_STORAGE__DATA_DIR=/var/lib/kmood
KMOOD_SERVER__CORS__ALLOW_ANY_ORIGIN=false
KMOOD_WORKER__LOOK_UP_WEATHER=false
```

A single underscore stays inside a key name, so `data_dir` is written `DATA_DIR`; a double underscore descends a section. Lists are arrays — `KMOOD_SERVER__CORS__ALLOWED_ORIGINS=["https://a.example","https://b.example"]` — and the media backend switches the same way, with `KMOOD_STORAGE__MEDIA__BACKEND=s3` beside `KMOOD_STORAGE__MEDIA__BUCKET` and `KMOOD_STORAGE__MEDIA__REGION`.

Secrets belong here rather than in a file: `auth.jwt_secret`, `push.vapid_private_key`, `provider.encryption_key`, and the S3 access and secret keys.

`KMOOD_CONFIG_FILE` names a file other than `config.toml`. Having no `config.toml` is fine, since the defaults are a complete configuration — but a file you name that does not exist is an error, as are a malformed file and a value of the wrong type. The reasoning is in [ADR 0013](docs/adr/0013-configuration-layers-defaults-file-environment.md).

## Push Notifications

K-Mood supports Web Push notifications (works on iOS 16.4+ when added to Home Screen, Android, and desktop browsers). No Firebase or third-party service required.

**1. Generate a VAPID private key** (32 bytes, base64url or standard base64):

```bash
python3 -c "
import subprocess, base64
key = subprocess.check_output(
    'openssl ecparam -genkey -name prime256v1 -noout 2>/dev/null | openssl ec -outform DER 2>/dev/null',
    shell=True
)
print(base64.urlsafe_b64encode(key[7:39]).rstrip(b'=').decode())
"
```

Standard base64 is accepted too: `+` and `/` are rewritten and any padding is stripped before use, so a key copied from a tool that emits the standard alphabet works unchanged.

**2. Add to `config.toml`:**

```toml
[push]
enabled = true
vapid_private_key = "<output from step 1>"
vapid_subject = "mailto:you@example.com"
```

Or keep the key out of the file entirely:

```bash
KMOOD_PUSH__ENABLED=true
KMOOD_PUSH__VAPID_PRIVATE_KEY=<output from step 1>
KMOOD_PUSH__VAPID_SUBJECT=mailto:you@example.com
```

**3. Enable in the app:** Go to Settings and tap "Enable" under Notifications. Use "Send test notification" to verify it works.

The **worker** checks reminders every 60 seconds and sends push notifications to all subscribed devices for users with due reminders. Users must set a timezone in their profile for reminders to fire — and the worker must be running, or nothing is sent.

## Architecture

Rust workspace with DDD and hexagonal architecture:

```
crates/
  domain/          Pure domain logic, entities, value objects, ports
  application/     Use cases as free-standing functions
  api-types/       Request/response DTOs and Zod-like validation
  config/          Configuration types, defaults, and the file/environment layering
  adapters/
    http-axum/     REST API (axum) + SPA serving
    sqlite/        SQLite persistence (sqlx)
    auth/          JWT + Argon2 authentication
    storage/       Media storage (local filesystem / S3)
    event-publisher/ Domain event bus (tokio mpsc)
    importer/      Daylio CSV, generic CSV, and backup reading
    exporter/      Complete backup (ZIP) and shareable journal (Markdown)
    music/         Subsonic now-playing, MusicBrainz recording lookup
    weather/       Open-Meteo lookup, WMO codes mapped to our own vocabulary
    crypto/        Provider credential encryption
    web-push/      Reminder delivery
  bootstrap/       Shared wiring, so both binaries build one object graph
  server/          The HTTP binary (k-mood)
  worker/          The background binary (k-mood-worker)
spa/               React 19 SPA (TanStack Router, shadcn/ui, Tailwind v4)
```

Two processes, one SQLite file. The server only serves; everything on a timer — reminders, session cleanup, enrichment — is the worker's. Background work is queued but the queue is deliberately losable: nothing is enqueued that a query over stored data cannot rediscover, so a lost job costs promptness and never data.

Decisions with reasoning worth keeping are in `docs/adr/`, and the domain vocabulary is in `CONTEXT.md`.

## API

Interactive API docs are available at `/docs` (Scalar UI) when the server is running. The OpenAPI spec is at `/openapi.json`.

## License

[MIT](LICENSE)
