.PHONY: build dev dev-server dev-worker check check-all test fmt run run-server run-worker clean fix spa

SERVER := k-mood
WORKER := k-mood-worker

build: spa
	cargo build --release

spa:
	cd spa && bun install && bun run build

# Runs the server with the worker beside it, which is how k-mood is meant to run:
# reminders, session cleanup and enrichment all live in the worker. Stopping the
# server stops both.
run: build
	@./target/release/$(WORKER) & \
	WORKER_PID=$$!; \
	trap "kill $$WORKER_PID 2>/dev/null || true" EXIT INT TERM; \
	./target/release/$(SERVER)

run-server: build
	./target/release/$(SERVER)

run-worker: build
	./target/release/$(WORKER)

dev:
	@cargo build --bin $(SERVER) --bin $(WORKER)
	@RUST_LOG=debug ./target/debug/$(WORKER) & \
	WORKER_PID=$$!; \
	trap "kill $$WORKER_PID 2>/dev/null || true" EXIT INT TERM; \
	RUST_LOG=debug ./target/debug/$(SERVER)

dev-server:
	RUST_LOG=debug cargo run --bin $(SERVER)

dev-worker:
	RUST_LOG=debug cargo run --bin $(WORKER)

# Lints tests and benches as well as the crates themselves: a warning in test
# code is still a warning, and half the codebase is tests.
check:
	cargo fmt --all -- --check
	cargo clippy --workspace --all-targets -- -D warnings
	cargo test --workspace
	cd spa && bun run check && bun run lint && bun run typecheck

check-all: check

test:
	cargo test --workspace

fmt:
	cargo fmt --all
	cd spa && bun run format

fix:
	cargo fmt --all
	cargo clippy --fix --workspace --all-targets --allow-dirty --allow-staged

clean:
	cargo clean
	rm -rf spa/dist

docker:
	docker build -t k-mood .
