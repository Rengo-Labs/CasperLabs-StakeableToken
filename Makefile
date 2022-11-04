wasm_src_path = target/wasm32-unknown-unknown/release
tests_wasm = tests/wasm
contract_build_command = make prepare && make build-contract

des_wasm_stakeable_token = stakeable-token/${tests_wasm}
des_wasm_liquidity_guard = liquidity-guard/${tests_wasm}
des_wasm_stable_usd_equivalent = stable-usd-equivalent/${tests_wasm}
des_wasm_transfer_helper = transfer-helper/${tests_wasm}

# Dependencies
uniswap_core_directory = ../CasperLabs-UniswapV2-core
uniswap_router_directory = ../Casperlabs-UniswapRouter
liquidity_transformer_directory = ../CasperLabs-Wise-LiquidityTransformer
# path to core contracts
erc20_contract = ${uniswap_core_directory}/erc20
factory_contract = ${uniswap_core_directory}/factory
flash_swapper_contract = ${uniswap_core_directory}/flashswapper
pair_contract = ${uniswap_core_directory}/pair
wcspr_contract = ${uniswap_core_directory}/wcspr
# paths to router contracts
library_contract = ${uniswap_router_directory}/uniswap-v2-library
router_contract = ${uniswap_router_directory}/uniswap-v2-router
# paths to stakeable contracts
stakeable_token_contract = ${stakeable_liquidity_transformer_directory}/

prepare:
	rustup target add wasm32-unknown-unknown

build-dependencies:
# build core contracts
	cd ${erc20_contract} && ${contract_build_command}
	cd ${factory_contract} && ${contract_build_command}
	cd ${flash_swapper_contract} && ${contract_build_command}
	cd ${pair_contract} && ${contract_build_command}
	cd ${wcspr_contract} && ${contract_build_command}
# build router contracts
	cd ${library_contract} && ${contract_build_command}
	cd ${router_contract} && ${contract_build_command}
# build transformer contracts
	cd ${liquidity_transformer_directory} && ${contract_build_command}

build-stakeable-token:
	cargo build --release -p stakeable-token -p stakeable-token-session-code --target wasm32-unknown-unknown
build-liquidity-guard:
	cargo build --release -p liquidity-guard -p liquidity-guard-session-code --target wasm32-unknown-unknown
build-stable-usd-equivalent:
	cargo build --release -p stable-usd-equivalent -p stable-usd-equivalent-session-code --target wasm32-unknown-unknown
build-transfer-helper:
	cargo build --release -p transfer-helper -p transfer-helper-session-code --target wasm32-unknown-unknown

copy-wasm-file-stakeable-token:
	cp ${router_contract}/${wasm_src_path}/uniswap-v2-router.wasm ${des_wasm_stakeable_token}
	cp ${factory_contract}/${wasm_src_path}/factory.wasm ${des_wasm_stakeable_token}
	cp ${pair_contract}/${wasm_src_path}/pair-token.wasm ${des_wasm_stakeable_token}
	cp ${erc20_contract}/${wasm_src_path}/erc20-token.wasm ${des_wasm_stakeable_token}
	cp ${library_contract}/${wasm_src_path}/uniswap-v2-library.wasm ${des_wasm_stakeable_token}
	cp ${wcspr_contract}/${wasm_src_path}/wcspr-token.wasm ${des_wasm_stakeable_token}
	cp ${flash_swapper_contract}/${wasm_src_path}/flashswapper-token.wasm ${des_wasm_stakeable_token}
	cp ${liquidity_transformer_directory}/${wasm_src_path}/scspr.wasm ${des_wasm_stakeable_token}
	cp ${liquidity_transformer_directory}/${wasm_src_path}/liquidity_transformer.wasm ${des_wasm_stakeable_token}
	cp ${wasm_src_path}/liquidity-guard.wasm ${des_wasm_stakeable_token}
	cp ${wasm_src_path}/stakeable-token.wasm ${des_wasm_stakeable_token}
copy-wasm-file-liquidity-guard:
	cp ${wasm_src_path}/liquidity-guard.wasm ${des_wasm_liquidity_guard}
	cp ${wasm_src_path}/liquidity-guard-session-code.wasm ${des_wasm_liquidity_guard}
copy-wasm-file-stable-usd-equivalent:
	cp ${wasm_src_path}/stable-usd-equivalent.wasm ${des_wasm_stable_usd_equivalent}
	cp ${wasm_src_path}/stable-usd-equivalent-session-code.wasm ${des_wasm_stable_usd_equivalent}
copy-wasm-file-transfer-helper:
	cp ${wasm_src_path}/transfer-helper.wasm ${des_wasm_transfer_helper}
	cp ${wasm_src_path}/transfer-helper-session-code.wasm ${des_wasm_transfer_helper}

test-stakeable-token:
	cargo test -p stakeable-token-tests
test-liquidity-guard:
	cargo test -p liquidity-guard-tests
test-stable-usd-equivalent:
	cargo test -p stable-usd-equivalent-tests
test-transfer-helper:
	cargo test -p transfer-helper-tests

run-stakeable-token:
	make build-stakeable-token && make copy-wasm-file-stakeable-token && make test-stakeable-token
run-liquidity-guard:
	make build-liquidity-guard && make copy-wasm-file-liquidity-guard && make test-liquidity-guard
run-stable-usd-equivalent:
	make build-stable-usd-equivalent && make copy-wasm-file-stable-usd-equivalent && make test-stable-usd-equivalent
run-transfer-helper:
	make build-transfer-helper && make copy-wasm-file-transfer-helper && make test-transfer-helper

build-all:
	make build-stakeable-token
	make build-liquidity-guard
	make build-stable-usd-equivalent
	make build-transfer-helper

test-all:
	make test-stakeable-token
	make test-liquidity-guard
	make test-stable-usd-equivalent
	make test-transfer-helper

run-all:
	make run-stakeable-token
	make run-liquidity-guard
	make run-stable-usd-equivalent
	make run-transfer-helper

clean:
	cargo clean
	rm -rf ${des_wasm_stakeable_token}/*.wasm
	rm -rf ${des_wasm_liquidity_guard}/*.wasm
	rm -rf ${des_wasm_stable_usd_equivalent}/*.wasm
	rm -rf ${des_wasm_transfer_helper}/*.wasm

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

git-clean:
	git rm -rf --cached .
	git add .