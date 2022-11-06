use casper_types::{runtime_args, Key, RuntimeArgs, U256};
use casperlabs_test_env::TestEnv;
use tests_common::{
    deploys::deploy_liquidity_guard,
    helpers::{call, now, result_dict},
    keys::*,
};

#[test]
fn assign_inflation() {
    let env = TestEnv::new();
    let owner = env.next_user();
    let liquidity_guard = deploy_liquidity_guard(&env, owner, now());
    let ret: U256 = result_dict(
        &env,
        liquidity_guard.contract_hash(),
        INFLATION_LN,
        103000.to_string(),
    );
    assert_eq!(ret, 0.into(), "Inflation value already assigned");
    liquidity_guard.call_contract(owner, "assign_inflation", runtime_args! {}, 0);
    let ret: U256 = result_dict(
        &env,
        liquidity_guard.contract_hash(),
        INFLATION_LN,
        103000.to_string(),
    );
    assert_eq!(ret, 123477676u64.into(), "Inflation value not assigned");
}

#[test]
fn get_inflation() {
    let env = TestEnv::new();
    let owner = env.next_user();
    let liquidity_guard = deploy_liquidity_guard(&env, owner, now());
    call(
        &env,
        owner,
        SESSION_WASM_LIQUIDITY_GUARD,
        runtime_args! {
            ENTRYPOINT => "get_inflation",
            PACKAGE_HASH => Key::Hash(liquidity_guard.package_hash()),
            "amount" => U256::from(101314)
        },
        now(),
    );
    let ret: U256 = result_dict(
        &env,
        liquidity_guard.contract_hash(),
        INFLATION_LN,
        101314.to_string(),
    );
    assert_eq!(ret, 279593807u64.into(), "Invalid Inflation value");
}
