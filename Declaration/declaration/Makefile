prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p declaration --target wasm32-unknown-unknown
	wasm-strip declaration/target/wasm32-unknown-unknown/release/declaration.wasm 2>/dev/null | true

test-only:
	cargo test -p declaration_tests
	
copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/*.wasm declaration_tests/wasm

test: build-contract copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

clean:
	cargo clean
	rm -rf declaration_tests/wasm/*.wasm
