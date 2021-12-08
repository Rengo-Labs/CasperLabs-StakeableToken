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

// Keys for global struct
pub const TOTAL_STAKED: &str = "total_staked";
pub const TOTAL_SHARES: &str = "total_shares";
pub const SHARE_PRICE: &str = "share_price";
pub const CURRENT_WISE_DAY: &str = "current_wise_day";
pub const REFERRAL_SHARES: &str = "referral_shares";
pub const LIQUIDITY_SHARES: &str = "liquidity_shares";

pub struct SnapshotInstance(TestContract);
impl SnapshotInstance {
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: Sender,
        timing_hash: Key,
        declaration_hash: Key,
        globals_hash: Key,
        helper_contract_hash: Key,
        sbnb_contract_hash: Key,
        pair_contract_hash: Key,
        bep20_contract_hash: Key,
        guard_contract_hash: Key,
    ) -> TestContract {
        TestContract::new(
            env,
            "snapshot.wasm",
            contract_name,
            sender,
            runtime_args! {
                "timing" => timing_hash,
                "declaration" => declaration_hash,
                "globals"=> globals_hash,
                "helper" => helper_contract_hash,
                "sbnb"=>sbnb_contract_hash,
                "pair"=>pair_contract_hash,
                "bep20"=>bep20_contract_hash,
                "guard"=>guard_contract_hash
            },
        )
    }

    pub fn instance(contract: TestContract) -> SnapshotInstance {
        SnapshotInstance(contract)
    }

    pub fn manual_daily_snapshot(&self, sender: Sender){
        self.0.call_contract(
            sender,
            "manual_daily_snapshot",
            runtime_args!{}
        );
    }

    pub fn manual_daily_snapshot_point(&self, sender: Sender, update_day: u64){
        self.0.call_contract(
            sender,
            "manual_daily_snapshot_point",
            runtime_args!{
                "update_day"=>update_day
            }
        );
    }

    pub fn liquidity_guard_trigger(&self, sender: Sender){
        self.0.call_contract(
            sender,
            "liquidity_guard_trigger",
            runtime_args!{}
        );
    }

    pub fn snapshot_trigger(&self, sender: Sender){
        self.0.call_contract(
            sender,
            "snapshot_trigger",
            runtime_args!{}
        );
    }

    pub fn current_wise_day(&self)->U256{
        self.0.query_dictionary("globals_struct", CURRENT_WISE_DAY.to_string()).unwrap()
    }
    pub fn get_struct_from_key(&self, sender: Sender, struct_name: &str, key: U256){
        self.0.call_contract(
            sender,
            "get_struct_from_key",
            runtime_args!{
                "struct_name"=>struct_name.to_string(),
                "key"=>key
            }
        );
    }

    pub fn set_struct_from_key(&self, sender: Sender,  struct_name: &str, key: U256, value: &str){
         self.0.call_contract(
            sender,
            "set_struct_from_key",
            runtime_args!{
                "struct_name"=>struct_name.to_string(),
                "key"=>key,
                "value"=>value.to_string()
            }
        );
    }

    pub fn set_key_by_name(&self, sender: Sender, name: String, key: Key) {
        self.0.call_contract(
            sender,
            SET_KEY_BY_NAME_ENTRYPOINT_NAME,
            runtime_args! {
                NAME_RUNTIME_ARG_NAME=>name,
                KEY_RUNTIME_ARG_NAME => key
            },
        );
    }

    // ======================================================= //
    pub fn package_hash_result(&self) -> ContractPackageHash {
        self.0.query_named_key(PACKAGE_HASH_KEY_NAME.to_string())
    }

    pub fn contract_hash_result(&self) -> ContractHash {
        self.0.query_named_key(CONTRACT_HASH_KEY_NAME.to_string())
    }

    pub fn self_hash(&self)-> Key{
        self.0.query_named_key("self_hash".to_string())
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
