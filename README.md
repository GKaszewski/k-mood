# K-Mood

Self-hosted mood tracking journal. Log your mood, activities, photos, and voice memos. Track trends, streaks, and correlations over time.

## Features

- **Mood tracking** with 5 discrete states (Awful, Bad, Meh, Good, Rad)
- **Activities** with custom categories
- **Rich entries** with markdown notes, photos, and voice memos
- **Analytics** including mood trends, streaks, distribution, activity correlations, and calendar heatmap
- **Import/Export** with Daylio CSV preset, generic CSV wizard, and full ZIP backup
- **Multi-user** with JWT authentication and per-user data isolation
- **PWA** installable on mobile and desktop
- **Self-hosted** with SQLite and local or S3 media storage

## Quick Start

### Docker

```bash
docker run -d \
  -p 3000:3000 \
  -v k-mood-data:/data \
  -e KMOOD_AUTH__JWT_SECRET=your-secret-here \
  ghcr.io/gabrielkaszewski/k-mood:latest
```

Open `http://localhost:3000`, register an account, and start logging.

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

volumes:
  k-mood-data:
```

### From Source

Requires Rust 1.85+ and Bun.

```bash
# Build frontend
cd spa && bun install && bun run build && cd ..

# Build and run
cargo run --release
```

## Configuration

Copy `config.example.toml` to `config.toml` and adjust as needed.

| Section | Key | Default | Description |
|---------|-----|---------|-------------|
| `server` | `host` | `0.0.0.0` | Bind address |
| `server` | `port` | `3000` | HTTP port |
| `server.cors` | `allow_any_origin` | `true` | CORS policy |
| `auth` | `jwt_secret` | `change-me-in-production` | JWT signing key |
| `auth` | `allow_registration` | `true` | Enable new user registration |
| `storage` | `data_dir` | `./data` | SQLite and media storage path |
| `storage.media` | `backend` | `local` | `local` or `s3` |

## Push Notifications

K-Mood supports Web Push notifications (works on iOS 16.4+ when added to Home Screen, Android, and desktop browsers). No Firebase or third-party service required.

**1. Generate VAPID keys:**

```bash
openssl ecparam -genkey -name prime256v1 -noout -out vapid_private.pem
openssl ec -in vapid_private.pem -outform PEM 2>/dev/null | base64
```

**2. Add to `config.toml`:**

```toml
[push]
enabled = true
vapid_private_key = "<base64 output from step 1>"
vapid_subject = "mailto:you@example.com"
```

**3. Enable in the app:** Go to Settings and tap "Enable" under Notifications. Use "Send test notification" to verify it works.

The server checks reminders every 60 seconds and sends push notifications to all subscribed devices for users with due reminders. Users must set a timezone in their profile for reminders to fire.

## Architecture

Rust workspace with DDD and hexagonal architecture:

```
crates/
  domain/          Pure domain logic, entities, value objects, ports
  application/     Use cases as free-standing functions
  api-types/       Request/response DTOs and Zod-like validation
  config/          Configuration types and defaults
  adapters/
    http-axum/     REST API (axum) + SPA serving
    sqlite/        SQLite persistence (sqlx)
    auth/          JWT + Argon2 authentication
    storage/       Media storage (local filesystem / S3)
    event-publisher/ Domain event bus (tokio mpsc)
    importer/      Daylio CSV + generic import parsing
    exporter/      ZIP export with media
  server/          Composition root, startup, graceful shutdown
spa/               React 19 SPA (TanStack Router, shadcn/ui, Tailwind v4)
```

## API

Interactive API docs are available at `/docs` (Scalar UI) when the server is running. The OpenAPI spec is at `/openapi.json`.

## License

[MIT](LICENSE)
