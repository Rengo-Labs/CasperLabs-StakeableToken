use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{contracts::ContractPackageHash, Key, U256};
use contract_utils::{get_key, set_key, Dict};
extern crate alloc;
use alloc::string::{String, ToString};

pub const SELF_HASH: &str = "self_hash";
pub const PACKAGE_HASH: &str = "package_hash";
pub const IS_READY: &str = "is_ready";
pub const INFLATION: &str = "inflation";

pub fn self_hash() -> Key {
    get_key(SELF_HASH).unwrap_or_revert()
}
pub fn set_self_hash(hash: Key) {
    set_key(SELF_HASH, hash);
}

pub fn package_hash() -> ContractPackageHash {
    get_key(PACKAGE_HASH).unwrap_or_revert()
}
pub fn set_package_hash(hash: ContractPackageHash) {
    set_key(PACKAGE_HASH, hash);
}

pub fn is_ready() -> bool {
    get_key(IS_READY).unwrap_or_revert()
}
pub fn set_is_ready(status: bool) {
    set_key(PACKAGE_HASH, status);
}

pub struct InflationLN {
    dict: Dict,
}
impl InflationLN {
    pub fn instance() -> InflationLN {
        InflationLN {
            dict: Dict::instance(INFLATION),
        }
    }

    pub fn init() {
        Dict::init(INFLATION)
    }

    pub fn get(&self, key: &U256) -> U256 {
        self.dict.get(&key.to_string()).unwrap_or_default()
    }

    pub fn set(&self, key: &U256, value: U256) {
        self.dict.set(&key.to_string(), value);
    }
}
