prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p stable_usd_equivalent --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/stable_usd_equivalent.wasm 2>/dev/null | true

build-test-contract:
	cargo build --release -p test --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/stable_usd_equivalent_test.wasm 2>/dev/null | true
	
test-only:
	cargo test -p stable_usd_equivalent_tests
	
copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/*.wasm stable_usd_equivalent_tests/wasm

test: build-contract build-test-contract copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

clean:
	cargo clean
	rm -rf stable_usd_equivalent_tests/wasm/*.wasm
