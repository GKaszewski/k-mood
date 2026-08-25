# K-Mood SPA

Frontend for K-Mood, a self-hosted mood tracking journal. Log moods, activities, photos, and voice memos. Track trends, streaks, and correlations over time.

## Stack

React 19, TanStack Router (file-based), TanStack Query, Axios, Zod, Tailwind v4, shadcn/ui, Recharts, Vite

## Getting started

```bash
bun install
bun dev          # http://localhost:5173
```

Set `VITE_API_URL` in `.env` to point at the Rust API (defaults to same origin).

## Scripts

| Command | Description |
|---------|-------------|
| `bun dev` | Dev server on port 5173 |
| `bun run build` | Production build |
| `bun run preview` | Preview production build |
| `bun run typecheck` | Type check without emitting |
| `bun test` | Run tests |
| `bun run lint` | Lint |
| `bun run format` | Format with Prettier |

## Project structure

```
src/
  api/           # Zod schemas + Axios client
  components/    # UI and feature components
  hooks/         # TanStack Query hooks
  routes/        # File-based routes (TanStack Router)
  lib/           # Utilities
```
