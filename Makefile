ifneq (,$(wildcard ./.env))
    include .env
    export
endif

.PHONY: build
build:
	@echo "Building guest..."
	@cargo build --manifest-path ./src/guest/Cargo.toml --target wasm32-wasi
	@echo "Building host..."
	@cargo build --manifest-path ./src/host/Cargo.toml

.PHONY: clean
clean:
	@echo "Cleaning up..."
	@cargo clean

.PHONY: generate
generate:
	@echo "Generating component..."
	@wasm-tools component new ./target/wasm32-wasi/debug/guest.wasm -o ./guest.component.wasm --adapt ./eng/adapters/wasi_snapshot_preview1.reactor.wasm

.PHONY: run
run:
	@echo "Launching host..."
	@cargo run --manifest-path ./src/host/Cargo.toml
