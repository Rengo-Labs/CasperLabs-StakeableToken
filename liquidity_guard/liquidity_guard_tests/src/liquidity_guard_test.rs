use crate::constants::*;
use crate::liquidity_guard_instance::LiquidityGuardInstance;
use casper_engine_test_support::AccountHash;
use casper_types::{runtime_args, ContractPackageHash, Key, RuntimeArgs, U256, U512};
use test_env::{Sender, TestContract, TestEnv};
use stakeable_token_utils::commons::key_names::*;

fn deploy_liquidity_guard(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        &env,
        "liquidity_guard.wasm",
        "liquidity_guard",
        Sender(owner),
        runtime_args! {},
    )
}

fn deploy() -> (AccountHash, TestContract, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();

    let guard = deploy_liquidity_guard(&env, owner);
    let guard_contract_hash = Key::Hash(guard.contract_hash());
    let proxy = LiquidityGuardInstance::proxy(&env, owner, guard_contract_hash);

    (owner, guard, proxy)
}

#[test]
fn test_deploy() {
    let (_, guard, _) = deploy();
    let value: u64 = 100000;
    let inflation = U256::from(60835153328 as u64);
    let ret: U256 = guard.query_dictionary(LIQUIDITY_GUARD_INFLATION_DICT, value.to_string()).unwrap();
    assert_eq!(ret, inflation);
}

#[test]
fn test_assign_inflation() {
    let (owner, guard, proxy) = deploy();

    // values and their inflations, as set in contract constructor
    let value1 = U256::from(101320);
    let inflation1: U256 = U256::from(278331162 as u64);

    let value2 = U256::from(103000);
    let inflation2: U256 = U256::from(123477676 as u64);

    guard.call_contract(Sender(owner), "assign_inflation", runtime_args! {});

    let ret1: U256 = guard.query_dictionary(LIQUIDITY_GUARD_INFLATION_DICT, value1.to_string()).unwrap();
    assert_eq!(ret1, inflation1);
}

#[test]
fn get_inflation(){
    let (owner, guard, proxy) = deploy();
    let value: u64 = 100000;
    let inflation = U256::from(60835153328 as u64);
    proxy.call_contract(Sender(owner), "get_inflation", runtime_args!{
        "amount"=>value
    });
    let ret: U256 = proxy.query_named_key("result".to_string());
    assert_eq!(inflation, ret);
}