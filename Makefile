wasm_src_path = target/wasm32-unknown-unknown/release
tests_wasm = tests/wasm

liquidity_guard_des_wasm = liquidity-guard/${tests_wasm}
stable_usd_equivalent_des_wasm = stable-usd-equivalent/${tests_wasm}
transfer_helper_des_wasm = transfer-helper/${tests_wasm}

prepare:
	rustup target add wasm32-unknown-unknown

build-liquidity-guard:
	cargo build --release -p liquidity-guard -p liquidity-guard-session-code --target wasm32-unknown-unknown
build-stable-usd-equivalent:
	cargo build --release -p stable-usd-equivalent -p stable-usd-equivalent-session-code --target wasm32-unknown-unknown
build-transfer-helper:
	cargo build --release -p transfer-helper -p transfer-helper-session-code --target wasm32-unknown-unknown

copy-wasm-file-liquidity-guard:
	cp ${wasm_src_path}/liquidity-guard.wasm ${liquidity_guard_des_wasm}
	cp ${wasm_src_path}/liquidity-guard-session-code.wasm ${liquidity_guard_des_wasm}
copy-wasm-file-stable-usd-equivalent:
	cp ${wasm_src_path}/stable-usd-equivalent.wasm ${stable_usd_equivalent_des_wasm}
	cp ${wasm_src_path}/stable-usd-equivalent-session-code.wasm ${stable_usd_equivalent_des_wasm}
copy-wasm-file-transfer-helper:
	cp ${wasm_src_path}/transfer-helper.wasm ${transfer_helper_des_wasm}
	cp ${wasm_src_path}/transfer-helper-session-code.wasm ${transfer_helper_des_wasm}

test-liquidity-guard:
	cargo test -p liquidity-guard-tests
test-stable-usd-equivalent:
	cargo test -p stable-usd-equivalent-tests
test-transfer-helper:
	cargo test -p transfer-helper-tests

run-liquidity-guard:
	make build-liquidity-guard && make copy-wasm-file-liquidity-guard && make test-liquidity-guard
run-stable-usd-equivalent:
	make build-stable-usd-equivalent && make copy-wasm-file-stable-usd-equivalent && make test-stable-usd-equivalent
run-transfer-helper:
	make build-transfer-helper && make copy-wasm-file-transfer-helper && make test-transfer-helper

build-all:
	make build-liquidity-guard
	make build-stable-usd-equivalent
	make build-transfer-helper

test-all:
	make test-liquidity-guard
	make test-stable-usd-equivalent
	make test-transfer-helper

run-all:
	make run-liquidity-guard
	make run-stable-usd-equivalent
	make run-transfer-helper

clean:
	cargo clean
	rm -rf ${liquidity_guard_des_wasm}/*.wasm
	rm -rf ${stable_usd_equivalent_des_wasm}/*.wasm
	rm -rf ${transfer_helper_des_wasm}/*.wasm

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

git-clean:
	git rm -rf --cached .
	git add .