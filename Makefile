wise_token_core_directory = ./

busd_equivalent_contract_path = ${wise_token_core_directory}BUSDEquivalent/busd_equivalent/
declaration_contract_path  = ${wise_token_core_directory}Declaration/declaration/
globals_contract_path  = ${wise_token_core_directory}Globals/globals/
helper_contract_path  = ${wise_token_core_directory}Helper/helper/
liquidity_guard_contract_path = ${wise_token_core_directory}LiquidityGuard/liquidity_guard/
liquidity_token_contract_path = ${wise_token_core_directory}LiquidityToken/liquidity_token/
referral_token_contract_path = ${wise_token_core_directory}"Referral Token"/"Referral Token"/
snapshot_contract_path = ${wise_token_core_directory}Snapshot/snapshot/
staking_token_contract_path = ${wise_token_core_directory}"Staking Token"/"Staking Token"/
timing_contract_path = ${wise_token_core_directory}Timing/timing/
transfer_helper_contract_path = ${wise_token_core_directory}TransferHelper/transfer_helper/
wise_token_contract_path = ${wise_token_core_directory}WiseToken/wisetoken/

wasm_src_path = target/wasm32-unknown-unknown/release/

wasm_dest_busd_equibalent_path = ${busd_equivalent_contract_path}busd_equivalent_tests/wasm/
wasm_dest_declaration_path = ${declaration_contract_path}declaration_tests/wasm/
wasm_dest_globals_path = ${globals_contract_path}globals_tests/wasm/
wasm_dest_helper_path = ${helper_contract_path}helper_tests/wasm/
wasm_dest_liquidity_guard_path = ${liquidity_guard_contract_path}liquidity_guard_tests/wasm/
wasm_dest_liquidity_token_path = ${liquidity_token_contract_path}liquidity_token_tests/wasm/
wasm_dest_referral_token_path = ${referral_token_contract_path}referral-token-tests/wasm/
wasm_dest_snapshot_path = ${snapshot_contract_path}snapshot_tests/wasm/
wasm_dest_staking_token_path = ${staking_token_contract_path}staking-token-tests/wasm/
wasm_dest_timing_path = ${timing_contract_path}timing_tests/wasm/
wasm_dest_transfer_helper_path = ${transfer_helper_contract_path}transfer_helper_tests/wasm/
wasm_dest_wise_token_path = ${wise_token_contract_path}wisetoken_tests/wasm/

all:
	# Build BUSDEquivalent
	cd ${busd_equivalent_contract_path} && make build-contract
	
	# Build Declaration
	cd ${declaration_contract_path} && make build-contract

	# Build Globals
	cd ${globals_contract_path} && make build-contract

	# Build Helper
	cd ${helper_contract_path} && make build-contract

	# Build LiquidityGuard
	cd ${liquidity_guard_contract_path} && make build-contract

	# Build LiquidityToken
	cd ${liquidity_token_contract_path} && make build-contract

	# Build Referral Token
	cd ${referral_token_contract_path} && make build-contract

	# Build Snapshot
	cd ${snapshot_contract_path} && make build-contract

	# Build Staking Token
	cd ${staking_token_contract_path} && make build-contract

	# Build Timing
	cd ${timing_contract_path} && make build-contract

	# Build TransferHelper
	cd ${transfer_helper_contract_path} && make build-contract

	# Build WiseToken
	cd ${wise_token_contract_path} && make build-contract

	# copy wasm files
	make copy-wasm-file

clean:
	# clean BUSDEquivalent
	cd ${busd_equivalent_contract_path} && make clean
	
	# clean Declaration
	cd ${declaration_contract_path} && make clean

	# clean Globals
	cd ${globals_contract_path} && make clean

	# clean Helper
	cd ${helper_contract_path} && make clean

	# clean LiquidityGuard
	cd ${liquidity_guard_contract_path} && clean

	# clean LiquidityToken
	cd ${liquidity_token_contract_path} && clean

	# clean Referral Token
	cd ${referral_token_contract_path} && clean

	# clean Snapshot
	cd ${snapshot_contract_path} && make clean

	# clean Staking Token
	cd ${staking_token_contract_path} && make clean

	# clean Timing
	cd ${timing_contract_path} && make clean

	# clean TransferHelper
	cd ${transfer_helper_contract_path} && make clean

	# clean WiseToken
	cd ${wise_token_contract_path} && make clean

	
# copy wasm to required directory
copy-wasm-file:


# run all tests sequentially
test:
	# make all contracts and test contracts
	make all

	# copy wasms to required locations
	make copy-wasm-files

	# test BUSDEquivalent
	cd ${busd_equivalent_contract_path} && make test
	
	# test Declaration
	cd ${declaration_contract_path} && make test

	# test Globals
	cd ${globals_contract_path} && make test

	# test Helper
	cd ${helper_contract_path} && make test

	# test LiquidityGuard
	cd ${liquidity_guard_contract_path} && test

	# test LiquidityToken
	cd ${liquidity_token_contract_path} && test

	# test Referral Token
	cd ${referral_token_contract_path} && test

	# test Snapshot
	cd ${snapshot_contract_path} && make clean

	# test Staking Token
	cd ${staking_token_contract_path} && make test

	# test Timing
	cd ${timing_contract_path} && make test

	# test TransferHelper
	cd ${transfer_helper_contract_path} && make test

	# test WiseToken
	cd ${wise_token_contract_path} && make test
