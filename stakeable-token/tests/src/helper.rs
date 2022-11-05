use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, Key, RuntimeArgs, U256, U512,
};
use casperlabs_test_env::{TestContract, TestEnv};
use std::time::SystemTime;

pub const MILLI_SECONDS_IN_DAY: u64 = 86_400_000;
pub const SCSPR_AMOUNT: U512 = U512([100_000_000_000, 0, 0, 0, 0, 0, 0, 0]);
pub const TRANSFORMER_AMOUNT: U512 = U512([100_000_000_000, 0, 0, 0, 0, 0, 0, 0]);
pub const TWOTHOUSEND_CSPR: U512 = U512([2_000_000_000_000, 0, 0, 0, 0, 0, 0, 0]);
pub const ONETHOUSEND_CSPR: U256 = U256([1_000_000_000_000, 0, 0, 0]);

pub fn zero_address() -> Key {
    Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000")
        .unwrap()
}

pub fn account_zero_address() -> Key {
    Key::from_formatted_str(
        "account-hash-0000000000000000000000000000000000000000000000000000000000000000",
    )
    .unwrap()
}

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

pub fn key_to_str(key: &Key) -> String {
    match key {
        Key::Account(account) => account.to_string(),
        Key::Hash(package) => hex::encode(package),
        _ => "".into(),
    }
}

pub fn session_code_call(
    env: &TestEnv,
    sender: AccountHash,
    runtime_args: RuntimeArgs,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "session-code-lt.wasm",
        "session-code-lt",
        sender,
        runtime_args,
        time,
    )
}

pub fn session_code_result<T: CLTyped + FromBytes>(
    env: &TestEnv,
    sender: AccountHash,
    key: &str,
) -> T {
    env.query_account_named_key(sender, &[key.into()])
}

pub fn deploy_uniswap_router(
    env: &TestEnv,
    owner: AccountHash,
    uniswap_factory: &TestContract,
    wcspr: &TestContract,
    uniswap_library: &TestContract,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "uniswap-v2-router.wasm",
        "uniswap-v2-router",
        owner,
        runtime_args! {
            "factory" => Key::Hash(uniswap_factory.package_hash()),
            "wcspr" => Key::Hash(wcspr.package_hash()),
            "library" => Key::Hash(uniswap_library.package_hash())
        },
        time,
    )
}

pub fn deploy_uniswap_factory(
    env: &TestEnv,
    owner: AccountHash,
    fee_to_setter: Key,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "factory.wasm",
        "factory",
        owner,
        runtime_args! {
            "fee_to_setter" => fee_to_setter
        },
        time,
    )
}

pub fn deploy_uniswap_pair(
    env: &TestEnv,
    owner: AccountHash,
    contract_name: &str,
    name: String,
    symbol: String,
    decimals: u8,
    initial_supply: U256,
    flash_swapper: &TestContract,
    uniswap_factory: &TestContract,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "pair-token.wasm",
        contract_name,
        owner,
        runtime_args! {
            "name" => name,
            "symbol" => symbol,
            "decimals" => decimals,
            "initial_supply" => initial_supply,
            "callee_package_hash" => Key::Hash(flash_swapper.package_hash()),
            "factory_hash" => Key::Hash(uniswap_factory.package_hash()),
        },
        time,
    )
}

pub fn deploy_erc20(
    env: &TestEnv,
    owner: AccountHash,
    name: String,
    symbol: String,
    decimals: u8,
    initial_supply: U256,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "erc20-token.wasm",
        "erc20",
        owner,
        runtime_args! {
            "name" => name,
            "symbol" => symbol,
            "decimals" => decimals,
            "initial_supply" => initial_supply
        },
        time,
    )
}

pub fn deploy_uniswap_library(env: &TestEnv, owner: AccountHash, time: u64) -> TestContract {
    TestContract::new(
        env,
        "uniswap-v2-library.wasm",
        "library",
        owner,
        runtime_args! {},
        time,
    )
}

pub fn deploy_wcspr(
    env: &TestEnv,
    owner: AccountHash,
    name: String,
    symbol: String,
    decimals: u8,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "wcspr-token.wasm",
        "wcspr",
        owner,
        runtime_args! {
            "name" => name,
            "symbol" => symbol,
            "decimals" => decimals
        },
        time,
    )
}

pub fn deploy_flash_swapper(
    env: &TestEnv,
    owner: AccountHash,
    wcspr: &TestContract,
    erc20: &TestContract,
    uniswap_factory: &TestContract,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "flashswapper-token.wasm",
        "flash_swapper",
        owner,
        runtime_args! {
            "wcspr" => Key::Hash(wcspr.package_hash()),
            "dai" => Key::Hash(erc20.package_hash()),
            "uniswap_v2_factory" => Key::Hash(uniswap_factory.package_hash())
        },
        time,
    )
}

pub fn deploy_liquidity_guard(env: &TestEnv, owner: AccountHash, time: u64) -> TestContract {
    TestContract::new(
        env,
        "liquidity-guard.wasm",
        "liquidity-guard",
        owner,
        runtime_args! {},
        time,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn deploy_scspr(
    env: &TestEnv,
    owner: AccountHash,
    wcspr: &TestContract,
    uniswap_pair: &TestContract,
    uniswap_router: &TestContract,
    uniswap_factory: &TestContract,
    amount: U512,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "scspr.wasm",
        "scspr",
        owner,
        runtime_args! {
            "wcspr" => Key::Hash(wcspr.package_hash()),
            "uniswap_pair" => Key::Hash(uniswap_pair.package_hash()),
            "uniswap_router" => Key::Hash(uniswap_router.package_hash()),
            "uniswap_factory" => Key::Hash(uniswap_factory.package_hash()),
            "amount" => amount
        },
        time,
    )
}

#[allow(clippy::new_ret_no_self, clippy::too_many_arguments)]
pub fn deploy_liquidity_transformer(
    env: &TestEnv,
    contract_name: &str,
    sender: AccountHash,
    stakeable: Key,
    scspr: Key,
    pair_stakeable: Key,
    pair_scspr: Key,
    uniswap_router: Key,
    wcspr: Key,
    amount: U512,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "liquidity_transformer.wasm",
        contract_name,
        sender,
        runtime_args! {
            "wise" => stakeable,
            "scspr" => scspr,
            "pair_wise" => pair_stakeable,
            "pair_scspr" => pair_scspr,
            "uniswap_router" => uniswap_router,
            "wcspr" => wcspr,
            "amount" => amount
        },
        time,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn deploy_stakeable(
    env: &TestEnv,
    owner: AccountHash,
    stable_usd: &TestContract,
    scspr: &TestContract,
    wcspr: &TestContract,
    uniswap_router: &TestContract,
    uniswap_factory: &TestContract,
    uniswap_pair: &TestContract,
    liquidity_guard: &TestContract,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "stakeable-token.wasm",
        "stakeable-token",
        owner,
        runtime_args! {
            "stable_usd" => Key::Hash(stable_usd.package_hash()),
            "scspr" => Key::Hash(scspr.package_hash()),
            "wcspr" => Key::Hash(wcspr.package_hash()),
            "uniswap_router" => Key::Hash(uniswap_router.package_hash()),
            "uniswap_factory" => Key::Hash(uniswap_factory.package_hash()),
            "uniswap_pair" => Key::Hash(uniswap_pair.package_hash()),
            "liquidity_guard" => Key::Hash(liquidity_guard.package_hash()),
        },
        time,
    )
}
