# Code Style Guide

This document defines the coding standards for this project. All contributors (human and agent) must follow these rules. When in doubt, prefer clarity and correctness over cleverness and speed.

## Philosophy

- Code reads like prose
- Do things properly — no shortcuts
- SOLID principles — always, no exceptions
- KISS — simplest solution that solves the problem **correctly and properly**. KISS means no unnecessary complexity, NOT skipping proper architecture. If the design calls for a trait, a newtype, or a pattern — that IS the simple solution. Cutting corners is not simplicity, it's technical debt. When in doubt, follow the architecture (CONTEXT.md, ADRs) over perceived simplicity.
- DRY — one source of truth for every piece of knowledge
- Use design patterns where they fit (Strategy, Observer, Builder, etc.) — name them in code so intent is clear
- Single responsibility everywhere: files, functions, types, modules
- Single point of entry, single control flow — no parallel data/control paths
- Open/Closed — extend via new types, don't modify existing ones
- Depend on abstractions (traits), not concretions
- Keep things as private as possible
- Zero magic numbers, strings, or unnamed constants
- Newtypes for self-documenting code
- Descriptive variable names — `track_duration`, not `td`. `replication_policy`, not `rp`. `content_hash`, not `ch`. No abbreviations unless universally understood (e.g., `id`, `url`, `i` for a trivial loop index).

## Project Structure

### Module Organization

Deep module hierarchy, grouped by concern. No flat file dumps. Each file is its own small, enclosed thing.

```
crates/domain/src/
├── track/
│   ├── track.rs
│   ├── track_id.rs
│   ├── duration.rs
│   ├── content_hash.rs
│   └── audio_format.rs
├── album/
│   ├── album.rs
│   ├── album_id.rs
│   └── track_listing.rs
├── artist/
│   ├── artist.rs
│   ├── artist_id.rs
│   └── artist_profile.rs
├── federation/
│   ├── activity.rs
│   ├── remote_instance.rs
│   └── follow.rs
├── streaming/
│   ├── playback_session.rs
│   ├── replication_policy.rs
│   └── content_availability.rs
└── ...
```

### File Size

Small files with a single concern. If a file grows past ~100-150 lines, it's probably doing too much — split it.

### Tests

**Never** in the same file as production code. Always in a sibling `tests/` directory mirroring the module structure.

```
crates/domain/src/track/track.rs
crates/domain/tests/track/track_test.rs
```

## Naming

### Conventions

- Files and modules: `snake_case`
- Types: `PascalCase` with **semantic suffixes** for clarity and navigation
- Functions and methods: `snake_case`, verb-first, descriptive

### Type Suffixes

| Suffix | Usage | Example |
|---|---|---|
| (none) | Value objects, entities | `Track`, `Album`, `Artist` |
| `Id` | Identity newtypes | `TrackId`, `AlbumId`, `ArtistId` |
| `Config` | Configuration data | `ReplicationConfig`, `StorageConfig` |
| `Service` | Domain services | `ContentDiscoveryService` |
| `Port` | Application port traits | `TrackRepositoryPort`, `P2pTransportPort` |
| `Adapter` | Port implementations | `IrohTransportAdapter`, `SqliteTrackAdapter` |
| `Error` | Error types | `FederationError`, `StreamingError` |
| `UseCase` | Application use cases | `UploadTrackUseCase`, `StreamTrackUseCase` |
| `Strategy` | Strategy trait/impls | `ReplicationStrategy`, `EagerReplicationStrategy` |

### Newtypes

Use newtypes everywhere a primitive carries domain meaning:

```rust
// YES
struct TrackId(Uuid);
struct AlbumId(Uuid);
struct ArtistId(Uuid);
struct ContentHash(Blake3Hash);
struct Duration(u64);  // milliseconds
struct ByteRange(u64, u64);
struct InstanceUrl(Url);

// NO
type TrackId = String;
fn stream_track(id: &str, start: u64, end: u64) -> ...
```

Newtypes are self-documenting and prevent accidental misuse (can't pass a `TrackId` where an `AlbumId` is expected).

## Functions and Methods

### Single Responsibility

Each function does ONE thing. Compose small functions into larger behaviors.

```rust
// YES
fn resolve_content_sources(...) -> Vec<ContentSource> { ... }
fn select_best_source(sources: &[ContentSource], ...) -> ContentSource { ... }
fn initiate_stream(source: &ContentSource, range: &ByteRange) -> StreamHandle { ... }

// NO
fn stream_track(...) -> StreamHandle {
    // 80 lines doing source resolution, selection, and stream initiation
}
```

### Visibility

Default to private. Only `pub` what the module's consumers actually need. Use `pub(crate)` for intra-crate sharing that shouldn't leak outside.

### impl Blocks

Split by concern:

```rust
// Construction
impl Track {
    pub fn new(title: TrackTitle, artist: ArtistId, duration: Duration, hash: ContentHash) -> Self { ... }
}

// Queries
impl Track {
    pub fn is_available_locally(&self) -> bool { ... }
    pub fn content_hash(&self) -> &ContentHash { ... }
}

// Mutations
impl Track {
    pub fn update_metadata(&mut self, metadata: TrackMetadata) { ... }
}
```

## Error Handling

### Domain and Application: `thiserror`

Typed, matchable error enums. Every error variant is meaningful.

```rust
#[derive(Debug, thiserror::Error)]
pub enum FederationError {
    #[error("instance {0} is unreachable")]
    InstanceUnreachable(InstanceUrl),
    #[error("activity rejected by remote instance: {0}")]
    ActivityRejected(String),
}
```

### Adapters and Bootstrap: `anyhow`

Pragmatic error handling. Context strings for debugging.

```rust
fn fetch_track_blob(hash: &ContentHash) -> anyhow::Result<Bytes> {
    let data = iroh_client.get(hash)
        .context("failed to fetch track blob from P2P network")?;
    // ...
}
```

### Never `unwrap` or `expect` in production code

Errors propagate with `?` and bubble up to a single handler. No scattered panic points.

`unwrap` is allowed **only** in test code.

## Configuration

Secrets (API keys, JWT secrets) live in environment variables. Tunable settings (replication policy, storage limits, federation parameters) live in a TOML config file.

```rust
// YES — in config
[replication]
policy = "eager"
max_storage_bytes = 10_737_418_240

// NO — magic numbers in code
if storage.used_bytes() > 10_737_418_240 { ... }
```

Zero magic numbers or strings. Everything named, everything explained by its name.

```rust
// YES
const MAX_TRACK_TITLE_LENGTH: usize = 256;
const DEFAULT_BYTE_RANGE_CHUNK: u64 = 1_048_576;

// NO
if title.len() > 256 { ... }
```

Constants are only for true invariants that never change. Tunable values belong in config.

## Comments

Zero comments unless explaining **why** something non-obvious is done.

```rust
// YES — explains a non-obvious constraint
// BLAKE3 hash, not SHA-256, because iroh uses BLAKE3 for content addressing
let hash = blake3::hash(&audio_data);

// NO — restates what the code does
// Create a new track with the given metadata
let track = Track::new(title, artist_id, duration, hash);

// NO — references the task/ticket
// Added for thesis requirement §3.2
```

## Dependencies

- Use good crates when they exist — don't reinvent
- Prefer crates with minimal transitive dependencies
- Aim for small memory footprint and executable size
- Domain crate: zero external dependencies (except `thiserror` and `serde` for derive)
- Application crate: depends only on domain
- Adapters: free to pull in platform crates (`axum`, `sqlx`, `iroh`, `nats`, etc.)
- Bootstrap: wiring only

## Generics Over Trait Objects

Use generics (static dispatch) for ports, not `dyn Trait`:

```rust
// YES — zero-cost, monomorphized
pub struct StreamTrackUseCase<T: P2pTransportPort, R: TrackRepositoryPort> {
    transport: T,
    repository: R,
}

// NO — heap allocation, dynamic dispatch overhead
pub struct StreamTrackUseCase {
    transport: Box<dyn P2pTransportPort>,
    repository: Box<dyn TrackRepositoryPort>,
}
```

## Testing

### Coverage Targets

| Crate | Approach | Target |
|---|---|---|
| Domain | TDD (red-green-refactor) | 100% |
| Application | TDD | High coverage |
| Adapters | Integration tests where practical | Best effort |
| Bootstrap | No tests | — |

### Test Organization

Tests live in a separate `tests/` directory, never inline:

```
crates/domain/
├── src/
│   └── track/
│       └── track.rs
└── tests/
    └── track/
        └── track_test.rs
```

### Test Naming

Tests read as specifications:

```rust
#[test]
fn uploading_track_stores_content_hash() { ... }

#[test]
fn eager_replication_forwards_to_all_followers() { ... }

#[test]
fn byte_range_request_returns_correct_audio_slice() { ... }
```

## Control Flow

### Single Point of Entry

One path through the code. No parallel flows that do similar things in different places.

```rust
// YES — one function handles content resolution
let source = content_resolver.resolve(track_id, requester);

// NO — content resolution duplicated in two places
// stream_handler.rs: resolves content source
// federation_handler.rs: also resolves content source with slightly different logic
```

### Single Call, Cascading Changes

When something changes, it should cascade from one point. Don't require updating multiple files for a single logical change.

### No God Objects

If a struct has more than 5-6 fields, question whether it's doing too much. If a function takes more than 4-5 parameters, consider grouping them into a dedicated type.
