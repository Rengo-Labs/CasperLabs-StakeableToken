use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, CLTyped, Key, RuntimeArgs, U256, U512,
};
use casperlabs_test_env::{TestContract, TestEnv};
use std::time::SystemTime;

pub const MILLI_SECONDS_IN_DAY: u64 = 86_400_000;
pub const SCSPR_AMOUNT: U512 = U512([100_000_000_000, 0, 0, 0, 0, 0, 0, 0]);
pub const TRANSFORMER_AMOUNT: U512 = U512([100_000_000_000, 0, 0, 0, 0, 0, 0, 0]);
pub const STAKEABLE_AMOUNT: U512 = U512([0, 0, 0, 0, 0, 0, 0, 0]);
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

pub fn call(
    env: &TestEnv,
    sender: AccountHash,
    wasm: &str,
    runtime_args: RuntimeArgs,
    time: u64,
) -> TestContract {
    TestContract::new(env, wasm, "call", sender, runtime_args, time)
}

pub fn result_key<T: CLTyped + FromBytes>(env: &TestEnv, sender: AccountHash, key: &str) -> T {
    env.query_account_named_key(sender, &[key.into()])
}

pub fn result_dict<T: CLTyped + FromBytes + Default>(
    env: &TestEnv,
    contract_hash: [u8; 32],
    dict_name: &str,
    key: String,
) -> T {
    env.query_dictionary(contract_hash, dict_name, key)
        .unwrap_or_default()
}
