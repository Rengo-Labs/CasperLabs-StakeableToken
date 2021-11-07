prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p i_busd_equivalent --target wasm32-unknown-unknown
	wasm-strip i_busd_equivalent/target/wasm32-unknown-unknown/release/i_busd_equivalent.wasm 2>/dev/null | true

test-only:
	cargo test -p i_busd_equivalent_tests
	
copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/*.wasm i_busd_equivalent/wasm

test: build-contract copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

clean:
	cargo clean
	rm -rf i_busd_equivalent/wasm/*.wasm
