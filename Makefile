# paths to other repos
uniswap_core_directory = ../CasperLabs-UniswapV2-core
uniswap_router_directory = ../Casperlabs-UniswapRouter
wise_liquidity_transformer_directory = ../CasperLabs-Wise-LiquidityTransformer
wise_liquidity_transformer_scspr_directory = ${wise_liquidity_transformer_directory}/scspr/SCSPR
wise_token_directory = .
erc20_directory = erc20

# path to core contracts
#erc20_contract = ${uniswap_core_directory}/erc20
factory_contract = ${uniswap_core_directory}/factory
flash_swapper_contract = ${uniswap_core_directory}/flash-swapper
pair_contract = ${uniswap_core_directory}/pair
wcspr_contract = ${uniswap_core_directory}/wcspr

# paths to router contracts
library_contract = ${uniswap_router_directory}/uniswap-v2-library
router_contract = ${uniswap_router_directory}/uniswap-v2-router

# paths to liquidity transformer contracts
liquidity_transformer_contract = ${wise_liquidity_transformer_directory}/LiquidityTransformer/LiquidityTransformer
scspr_contract = ${wise_liquidity_transformer_directory}/${wise_liquidity_transformer_scspr_directory}/scspr
synthetic_helper_contract = ${wise_liquidity_transformer_directory}/${wise_liquidity_transformer_scspr_directory}/SyntheticHelper
synthetic_token_contract = ${wise_liquidity_transformer_directory}/${wise_liquidity_transformer_scspr_directory}/SyntheticToken

# paths to wise contracts
wise_token_contract = ${wise_token_directory}/wisetoken
liquidity_guard_contract = ${wise_token_directory}/liquidity_guard
stable_usd_contract = ${wise_token_directory}/stable_usd
transfer_helper_contract = ${wise_token_directory}/transfer_helper

# path to test contract for crates tests
wise_crates_test_contract = ${wise_token_directory}/test-contract

# wasm source and dest paths for wise token repo
wasm_src_path = target/wasm32-unknown-unknown/release
wasm_dest_stable_usd = ${stable_usd_contract}/stable_usd_tests/wasm
wasm_dest_transfer_helper = ${transfer_helper_contract}/transfer_helper_tests/wasm
wasm_dest_wise_token = ${wise_token_contract}/wisetoken_tests/wasm
wasm_dest_liquidity_guard = ${liquidity_guard_contract}/liquidity_guard_tests/wasm
wasm_dest_wise_crates_test_contract = ${wise_crates_test_contract}/crate-test/wasm

# commands as variables
contract_build_command = make build-contract
test_contract_build_command = make build-test-contract

all:
# build erc20
	cd ${erc20_directory} && ${contract_build_command}
	
# build core contracts
#	cd ${erc20_contract} && ${contract_build_command}
	cd ${factory_contract} && ${contract_build_command}
	cd ${flash_swapper_contract} && ${contract_build_command}
	cd ${pair_contract} && ${contract_build_command}
	cd ${wcspr_contract} && ${contract_build_command}

# build router contracts
	cd ${library_contract} && ${contract_build_command}
	cd ${router_contract} && ${contract_build_command}

# build transformer contracts
	cd ${liquidity_transformer_contract} && ${contract_build_command}
	cd ${scspr_contract} && ${contract_build_command}
	cd ${synthetic_helper_contract} && ${contract_build_command}
	cd ${synthetic_token_contract} && ${contract_build_command}

# build wise contracts
	cd ${liquidity_guard_contract} && ${contract_build_command} && ${test_contract_build_command} 
	cd ${transfer_helper_contract} && ${contract_build_command} && ${test_contract_build_command}
	cd ${stable_usd_contract} && ${contract_build_command} && ${test_contract_build_command}
	cd ${wise_token_contract} && ${contract_build_command} && ${test_contract_build_command}
	cd ${wise_crates_test_contract} && ${contract_build_command}

# copy wasm files in place
	make copy-wasm-file

copy-wasm-file:
# copy erc20 wasms
	cp ${erc20_directory}/${wasm_src_path}/*.wasm ${wasm_dest_wise_token}
	cp ${erc20_directory}/${wasm_src_path}/*.wasm ${wasm_dest_stable_usd}
	cp ${erc20_directory}/${wasm_src_path}/*.wasm ${wasm_dest_transfer_helper}
	cp ${erc20_directory}/${wasm_src_path}/*.wasm ${wasm_dest_wise_crates_test_contract}
	
# copy router wasms
	cp ${router_contract}/${wasm_src_path}/*.wasm ${wasm_dest_stable_usd}
	cp ${router_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_token}
	cp ${router_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_crates_test_contract}
	
	cp ${library_contract}/${wasm_src_path}/*.wasm ${wasm_dest_stable_usd}
	cp ${library_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_token}
	cp ${library_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_crates_test_contract}

# copy core wasms
	cp ${factory_contract}/${wasm_src_path}/*.wasm ${wasm_dest_stable_usd}
	cp ${factory_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_token}
	cp ${factory_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_crates_test_contract}

	cp ${flash_swapper_contract}/${wasm_src_path}/*.wasm ${wasm_dest_stable_usd}
	cp ${flash_swapper_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_token}
	cp ${flash_swapper_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_crates_test_contract}

	cp ${wcspr_contract}/${wasm_src_path}/*.wasm ${wasm_dest_stable_usd}
	cp ${wcspr_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_token}
	cp ${wcspr_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_crates_test_contract}

	cp ${pair_contract}/${wasm_src_path}/*.wasm ${wasm_dest_stable_usd}
	cp ${pair_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_token}
	cp ${pair_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_crates_test_contract}

# copy wise liquidity transformer wasms
	cp ${liquidity_transformer_contract}/${wasm_src_path}/*.wasm ${wasm_dest_stable_usd}
	cp ${liquidity_transformer_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_token}
	cp ${liquidity_transformer_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_crates_test_contract}

	cp ${scspr_contract}/${wasm_src_path}/*.wasm ${wasm_dest_stable_usd}
	cp ${scspr_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_token}
	cp ${scspr_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_crates_test_contract}

	cp ${synthetic_helper_contract}/${wasm_src_path}/*.wasm ${wasm_dest_stable_usd}
	cp ${synthetic_helper_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_token}
	cp ${synthetic_helper_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_crates_test_contract}

	cp ${synthetic_token_contract}/${wasm_src_path}/*.wasm ${wasm_dest_stable_usd}
	cp ${synthetic_token_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_token}
	cp ${synthetic_token_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_crates_test_contract}

# copy wise token wasms 
	cp ${liquidity_guard_contract}/${wasm_src_path}/*.wasm ${wasm_dest_liquidity_guard}
	cp ${liquidity_guard_contract}/${wasm_src_path}/*.wasm ${wasm_dest_stable_usd}
	cp ${liquidity_guard_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_token}
	cp ${liquidity_guard_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_crates_test_contract}

	cp ${stable_usd_contract}/${wasm_src_path}/*.wasm ${wasm_dest_stable_usd}
	cp ${stable_usd_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_token}
	cp ${stable_usd_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_crates_test_contract}

	cp ${transfer_helper_contract}/${wasm_src_path}/*.wasm ${wasm_dest_transfer_helper}
	cp ${transfer_helper_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_token}
	cp ${transfer_helper_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_crates_test_contract}

	cp ${wise_token_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_token}
	cp ${wise_token_contract}/${wasm_src_path}/*.wasm ${wasm_dest_stable_usd}
	cp ${wise_token_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_crates_test_contract}

	cp ${wise_crates_test_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_crates_test_contract}
	cp ${wise_crates_test_contract}/${wasm_src_path}/*.wasm ${wasm_dest_wise_crates_test_contract}

clean:
# clean core contracts
	cd ${erc20_contract} && make clean
	cd ${factory_contract} && make clean
	cd ${flash_swapper_contract} && make clean
	cd ${pair_contract} && make clean
	cd ${wcspr_contract} && make clean
	cd ${erc20_directory} && make clean

# clean router contracts
	cd ${library_contract} && make clean
	cd ${router_contract} && make clean

# clean transformer contracts
	cd ${liquidity_transformer_contract} && make clean
	cd ${scspr_contract} && make clean
	cd ${synthetic_helper_contract} && make clean
	cd ${synthetic_token_contract} && make clean

# clean wise contracts
	cd ${liquidity_guard_contract} && make clean
	cd ${transfer_helper_contract} && make clean
	cd ${stable_usd_contract} && make clean
	cd ${wise_token_contract} && make clean
	cd ${wise_crates_test_contract} && make clean


test:
	make all
	cd ${liquidity_guard_contract} && make test 
	cd ${stable_usd_contract} && make test
	cd ${transfer_helper_contract} && make test
	cd ${wise_crates_test_contract} && make test
	cd ${wise_token_contract} && make test
