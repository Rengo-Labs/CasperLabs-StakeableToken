prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p wisetoken --target wasm32-unknown-unknown
	wasm-strip wisetoken/target/wasm32-unknown-unknown/release/wisetoken.wasm 2>/dev/null | true

build-test-contract:
	cargo build --release -p contract --target wasm32-unknown-unknown
	wasm-strip wisetoken/target/wasm32-unknown-unknown/release/contract.wasm 2>/dev/null | true
test-only:
	cargo test -p wisetoken_tests
	
copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/*.wasm wisetoken_tests/wasm

test: build-contract build-test-contract copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

clean:
	cargo clean
	rm -rf wisetoken_tests/wasm/*.wasm
