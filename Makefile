build: build-guest build-host

build-guest:
	@echo "Building guest..."
	@cargo build --manifest-path ./src/guest/Cargo.toml --target wasm32-wasi

build-host:
	@echo "Building host..."
	@cargo build --manifest-path ./src/host/consumer/Cargo.toml
	@cargo build --manifest-path ./src/host/producer/Cargo.toml

clean:
	@echo "Cleaning up..."
	@cargo clean

generate:
	@echo "Generating component..."
	@wasm-tools component new ./target/wasm32-wasi/debug/guest.wasm -o ./guest.component.wasm --adapt ./eng/adapters/wasi_snapshot_preview1.reactor.wasm

run-consumer:
	@echo "Launching host..."
	@cargo run --manifest-path ./src/host/consumer/Cargo.toml

run-producer:
	@echo "Launching host..."
	@cargo run --manifest-path ./src/host/producer/Cargo.toml
