use crate::constants::*;
use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_types::{
    bytesrepr::ToBytes, runtime_args, ContractHash, ContractPackageHash, Key, RuntimeArgs, URef,
    U256, U512,
};
use test_env::{Sender, TestContract, TestEnv};
use wise_token_utils::commons::key_names;

pub struct BUSDEquivalentInstance(TestContract);
impl BUSDEquivalentInstance {
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: Sender,
        wise: Key,
        scspr: Key,
        wcspr: Key,
        busd: Key,
        router: Key,
        factory: Key,
    ) -> TestContract {
        TestContract::new(
            env,
            "busd_equivalent.wasm",
            contract_name,
            sender,
            runtime_args! {
                  "wise" =>wise,
                  "scspr" =>scspr,
                  "wcspr"=>wcspr,
                  "busd" =>busd,
                  "router"=>router,
                  "factory" =>factory,
            },
        )
    }

    pub fn instance(contract: TestContract) -> BUSDEquivalentInstance {
        BUSDEquivalentInstance(contract)
    }

    pub fn get_busd_equivalent(&self, sender: Sender) {
        self.0
            .call_contract(sender, "get_busd_equivalent", runtime_args! {});
    }

    pub fn update_busd_equivalent(&self, sender: Sender) {
        self.0
            .call_contract(sender, "update_busd_equivalent", runtime_args! {});
    }

    pub fn balance_of<T: Into<Key>>(&self, token: &TestContract, account: T) -> U256 {
        token
            .query_dictionary("balances", key_to_str(&account.into()))
            .unwrap_or_default()
    }

    pub fn add_liquidity(
        &self,
        sender: Sender,
        token_a: Key,
        token_b: Key,
        amount_a_desired: U256,
        amount_b_desired: U256,
        amount_a_min: U256,
        amount_b_min: U256,
        to: Key,
        deadline: U256,
        pair: Option<Key>,
        router: Key,
    ) {
        self.0.call_contract(
            sender,
            "add_liquidity",
            runtime_args! {
                "token_a" => token_a,
                "token_b" => token_b,
                "amount_a_desired" => amount_a_desired,
                "amount_b_desired" => amount_b_desired,
                "amount_a_min" => amount_a_min,
                "amount_b_min" => amount_b_min,
                "to" => to,
                "deadline" => deadline,
                "pair" => pair,
                "router_hash" => router,
            },
        );
    }

    pub fn add_liquidity_cspr(
        &self,
        sender: Sender,
        token: Key,
        amount_token_desired: U256,
        amount_cspr_desired: U256,
        amount_token_min: U256,
        amount_cspr_min: U256,
        to: Key,
        deadline: U256,
        pair: Option<Key>,
        router: Key,
        test_contract_hash: Key,
    ) {
        self.0.call_contract(
            sender,
            "add_liquidity_cspr",
            runtime_args! {
                "token" => token,
                "amount_token_desired" => amount_token_desired,
                "amount_cspr_desired" => amount_cspr_desired,
                "amount_token_min" => amount_token_min,
                "amount_cspr_min" => amount_cspr_min,
                "to" => to,
                "deadline" => deadline,
                "pair" => pair,
                "router_hash" => router,
                "self_hash" => test_contract_hash
            },
        );
    }
    pub fn proxy(env: &TestEnv, sender: Sender, busd_equivalent: Key) -> TestContract {
        TestContract::new(
            env,
            "busd_equivalent_test.wasm",
            "proxy_contract",
            sender,
            runtime_args! {
                "busd_equivalent"=>busd_equivalent
            },
        )
    }

    pub fn set_key_by_name(&self, sender: Sender, name: String, key: Key) {
        self.0.call_contract(
            sender,
            "set_key_by_name",
            runtime_args! {
                "name"=>name,
                "key" => key
            },
        );
    }

    // ================== ERC20 Methods ==================== //
    pub fn transfer<T: Into<Key>>(&self, sender: Sender, recipient: T, amount: U256) {
        self.0.call_contract(
            sender,
            "transfer",
            runtime_args! {
                "recipient" => recipient.into(),
                "amount" => amount
            },
        );
    }

    // pub fn balance_of<T: Into<Key>>(&self, account: T) -> U256 {
    //     self.0
    //         .query_dictionary("balances", key_to_str(&account.into()))
    //         .unwrap_or_default()
    // }

    // ======================================================= //
    pub fn package_hash_result(&self) -> ContractPackageHash {
        self.0
            .query_named_key(key_names::SELF_PACKAGE_HASH.to_string())
    }

    pub fn contract_hash_result(&self) -> Key {
        self.0
            .query_named_key(key_names::SELF_CONTRACT_HASH.to_string())
    }

    pub fn get_busd_equivalent_result(&self) -> U256 {
        self.0
            .query_named_key("get_busd_equivalent_result".to_string())
    }

    pub fn get_update_busd_equivalent_result(&self) -> U256 {
        self.0
            .query_named_key(key_names::BUSD_EQ_LATEST_BUSD_EQUIVALENT.to_string())
    }
}

pub fn key_to_str(key: &Key) -> String {
    match key {
        Key::Account(account) => account.to_string(),
        Key::Hash(package) => hex::encode(package),
        _ => panic!("Unexpected key type"),
    }
}

pub fn keys_to_str(key_a: &Key, key_b: &Key) -> String {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(key_a.to_bytes().unwrap());
    hasher.update(key_b.to_bytes().unwrap());
    let mut ret = [0u8; 32];
    hasher.finalize_variable(|hash| ret.clone_from_slice(hash));
    hex::encode(ret)
}
