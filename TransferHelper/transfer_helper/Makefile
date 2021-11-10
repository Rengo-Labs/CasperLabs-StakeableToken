prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p transfer_helper --target wasm32-unknown-unknown
	wasm-strip transfer_helper/target/wasm32-unknown-unknown/release/transfer_helper.wasm 2>/dev/null | true

test-only:
	cargo test -p transfer_helper_tests
	
copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/*.wasm transfer_helper/wasm

test: build-contract copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

clean:
	cargo clean
	rm -rf transfer_helper/wasm/*.wasm
