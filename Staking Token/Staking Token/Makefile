prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p staking-token --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/staking-token-main.wasm 2>/dev/null | true

build-test-contract:
	cargo build --release -p test --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/test-token.wasm 2>/dev/null | true

test-only:
	cargo test -p staking-token-tests

copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/*.wasm staking-token-tests/wasm

test: build-contract build-test-contract copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

clean:
	cargo clean
	rm -rf staking-token-tests/wasm/*.wasm
