FROM oven/bun:1 AS frontend
WORKDIR /app/spa
COPY spa/package.json spa/bun.lock ./
RUN bun install --frozen-lockfile
COPY spa/ .
RUN bun run build

FROM rust:1-slim-bookworm AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/

RUN cargo build --release --bin k-mood

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates wget libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/k-mood /usr/local/bin/k-mood
COPY --from=frontend /app/spa/dist /spa/dist

RUN mkdir -p /data
VOLUME /data
WORKDIR /data

EXPOSE 3000
ENTRYPOINT ["k-mood"]
