.PHONY: build dev check check-all test fmt run clean fix spa

build: spa
	cargo build --release

dev:
	RUST_LOG=debug cargo run

spa:
	cd spa && bun install && bun run build

check:
	cargo fmt --all -- --check
	cargo clippy -- -D warnings
	cargo test
	cd spa && bun run check && bun run typecheck

check-all:
	cargo fmt --all -- --check
	cargo clippy --workspace -- -D warnings
	cargo test --workspace
	cd spa && bun run check && bun run typecheck

test:
	cargo test --workspace

fmt:
	cargo fmt --all
	cd spa && bun run format

run: spa
	cargo run --release

fix:
	cargo fmt --all
	cargo clippy --fix --allow-dirty --allow-staged

clean:
	cargo clean
	rm -rf spa/dist

docker:
	docker build -t k-mood .
