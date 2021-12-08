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

pub struct BUSDEquivalentInstance(TestContract);
impl BUSDEquivalentInstance {
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: Sender,
        wise: Key,
        sbnb: Key,
        wbnb: Key,
        busd: Key,
        router: Key,
        declaration: Key,
        factory: Key,
    ) -> TestContract {
        TestContract::new(
            env,
            "busd_equivalent",
            contract_name,
            sender,
            runtime_args! {
                  "wise" =>wise,
                  "sbnb" =>sbnb,
                  "wbnb"=>wbnb,
                  "busd" =>busd,
                  "router"=>router,
                  "declaration" =>declaration,
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
    
    pub fn proxy(env: &TestEnv, sender: Sender, busd_equivalent: Key) -> TestContract {
        TestContract::new(
            env,
            "contract.wasm",
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

    pub fn balance_of<T: Into<Key>>(&self, account: T) -> U256 {
        self.0
            .query_dictionary("balances", key_to_str(&account.into()))
            .unwrap_or_default()
    }

    // ======================================================= //
    pub fn package_hash_result(&self) -> ContractPackageHash {
        self.0.query_named_key(PACKAGE_HASH_KEY_NAME.to_string())
    }

    pub fn contract_hash_result(&self) -> ContractHash {
        self.0.query_named_key(CONTRACT_HASH_KEY_NAME.to_string())
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
