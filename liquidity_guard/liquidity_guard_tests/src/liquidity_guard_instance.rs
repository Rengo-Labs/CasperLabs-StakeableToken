use crate::constants::*;
use casper_engine_test_support::AccountHash;
use casper_types::{
    bytesrepr::ToBytes, runtime_args, ContractHash, ContractPackageHash, Key, RuntimeArgs, URef,
    U256, U512,
};
use test_env::{Sender, TestContract, TestEnv};
use stakeable_token_utils::commons::key_names;

pub struct LiquidityGuardInstance(TestContract);
impl LiquidityGuardInstance {
    pub fn new(env: &TestEnv, contract_name: &str, sender: Sender) -> TestContract {
        TestContract::new(
            env,
            "liquidity_guard.wasm",
            contract_name,
            sender,
            runtime_args! {},
        )
    }

    pub fn instance(contract: TestContract) -> LiquidityGuardInstance {
        LiquidityGuardInstance(contract)
    }
    pub fn proxy(env: &TestEnv, sender: AccountHash, liquidity_guard: Key) -> TestContract {
        TestContract::new(
            env,
            "liquidity_guard_test.wasm",
            "proxy_contract",
            Sender(sender),
            runtime_args! {
                "liquidity_guard"=>liquidity_guard
            },
        )
    }

    // ======================================================= //
}
