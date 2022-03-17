prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p stakeabletoken --target wasm32-unknown-unknown
	wasm-strip stakeabletoken/target/wasm32-unknown-unknown/release/stakeabletoken.wasm 2>/dev/null | true

build-test-contract:
	cargo build --release -p contract --target wasm32-unknown-unknown
	wasm-strip stakeabletoken/target/wasm32-unknown-unknown/release/contract.wasm 2>/dev/null | true
test-only:
	cargo test -p stakeabletoken_tests
	
copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/*.wasm stakeabletoken_tests/wasm

test: build-contract build-test-contract copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

clean:
	cargo clean
	rm -rf stakeabletoken_tests/wasm/*.wasm
