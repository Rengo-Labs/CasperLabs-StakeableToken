use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_types::{bytesrepr::ToBytes, runtime_args, Key, RuntimeArgs, U128, U256};
use test_env::{Sender, TestContract, TestEnv};

pub struct REFERRALTOKENInstance(TestContract);

impl REFERRALTOKENInstance {
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: Sender,
        timing_hash: Key,
        helper_hash: Key,
        bep20_hash: Key,
        snapshot_hash: Key,
        declaration_hash: Key,
    ) -> REFERRALTOKENInstance {
        REFERRALTOKENInstance(TestContract::new(
            env,
            "referral-token.wasm",
            contract_name,
            sender,
            runtime_args! {
                "declaration_hash" => declaration_hash,
                "timing_hash" => timing_hash,
                "helper_hash" => helper_hash,
                "bep20_hash" => bep20_hash,
                "snapshot_hash" => snapshot_hash,
            },
        ))
    }

    pub fn constructor(
        &self,
        sender: Sender,
        timing_hash: Key,
        helper_hash: Key,
        bep20_hash: Key,
        snapshot_hash: Key,
        declaration_hash: Key,
    ) {
        self.0.call_contract(
            sender,
            "constructor",
            runtime_args! {
                "declaration_hash" => declaration_hash,
                "timing_hash" => timing_hash,
                "helper_hash" => helper_hash,
                "bep20_hash" => bep20_hash,
                "snapshot_hash" => snapshot_hash,
            },
        );
    }
    pub fn add_referrer_shares_to_end<T: Into<Key>>(
        &self,
        sender: Sender,
        final_day: U256,
        shares: U256,
    ) {
        self.0.call_contract(
            sender,
            "add_referrer_shares_to_end",
            runtime_args! {
                "final_day" => final_day,
                "shares" => shares
            },
        );
    }

    pub fn remove_referrer_shares_to_end<T: Into<Key>>(
        &self,
        sender: Sender,
        final_day: U256,
        shares: U256,
    ) {
        self.0.call_contract(
            sender,
            "remove_referrer_shares_to_end",
            runtime_args! {
                "final_day" => final_day,
                "shares" => shares
            },
        );
    }

    pub fn add_critical_mass<T: Into<Key>>(
        &self,
        sender: Sender,
        referrer: T,
        dai_equivalent: U256,
    ) {
        self.0.call_contract(
            sender,
            "add_critical_mass",
            runtime_args! {
                "referrer" => referrer.into(),
                "dai_equivalent" => dai_equivalent
            },
        );
    }
    pub fn remove_critical_mass<T: Into<Key>>(
        &self,
        sender: Sender,
        referrer: T,
        dai_equivalent: U256,
        satrt_day: U256,
    ) {
        self.0.call_contract(
            sender,
            "remove_critical_mass",
            runtime_args! {
                "referrer" => referrer.into(),
                "dai_equivalent" => dai_equivalent,
                "satrt_day"=>satrt_day
            },
        );
    }

    // pub fn name(&self) -> String {
    //     self.0.query_named_key(String::from("name"))
    // }
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
