FROM oven/bun:1 AS frontend
WORKDIR /app/spa
COPY spa/package.json spa/bun.lock ./
RUN bun install --frozen-lockfile
COPY spa/ .
RUN bun run build

FROM rust:1-alpine AS builder
WORKDIR /app
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/

RUN cargo build --release --bin k-mood

FROM alpine:3
RUN apk add --no-cache ca-certificates
COPY --from=builder /app/target/release/k-mood /usr/local/bin/k-mood
COPY --from=frontend /app/spa/dist /spa/dist

RUN mkdir -p /data
VOLUME /data
WORKDIR /data

EXPOSE 3000
ENTRYPOINT ["k-mood"]
