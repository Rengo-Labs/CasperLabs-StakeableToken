wasm_src_path = target/wasm32-unknown-unknown/release/

curve_token_v3_des_wasm = curve-token-v3/curve-token-v3-tests/wasm
erc20_des_wasm = erc20/erc20-tests/wasm

prepare:
	rustup target add wasm32-unknown-unknown

build-liquidity-gauge-reward-wrapper-session-code:
	cargo build --release -p liquidity-gauge-reward-wrapper-session-code --target wasm32-unknown-unknown
build-i-reward-distribution-recipient:
	cargo build --release -p session-code -p i-reward-distribution-recipient --target wasm32-unknown-unknown

test-only-curve-token-v3:
	cargo test -p curve-token-v3-tests
test-only-erc20:
	cargo test -p erc20-tests

copy-wasm-file-curve-token-v3:
	cp ${wasm_src_path}/curve-token-v3.wasm ${curve_token_v3_des_wasm}
	cp ${wasm_src_path}/crv3-proxy-token.wasm ${curve_token_v3_des_wasm}
	cp ${wasm_src_path}/erc20-token.wasm ${curve_token_v3_des_wasm}
	cp ${wasm_src_path}/curve-rewards.wasm ${curve_token_v3_des_wasm}
copy-wasm-file-erc20:
	cp ${wasm_src_path}/erc20-proxy-token.wasm ${erc20_des_wasm}
	cp ${wasm_src_path}/erc20-token.wasm ${erc20_des_wasm}

test-gauge-proxy:
	make build-contract-gauge-proxy && make copy-wasm-file-gauge-proxy && make test-only-gauge-proxy
test-liquidity-gauge-reward:
	make build-contract-liquidity-gauge-reward && make copy-wasm-file-liquidity-gauge-reward && make test-only-liquidity-gauge-reward

all:
	make test-curve-token-v3
	make test-erc20
	make test-erc20-crv

clean:
	cargo clean
	rm -rf ${curve_token_v3_des_wasm}/*.wasm
	rm -rf ${erc20_des_wasm}/*.wasm

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

git-clean:
	git rm -rf --cached .
	git add .