use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{contracts::ContractPackageHash, Key, U256};
use contract_utils::{get_key, set_key, Dict};

pub const SELF_HASH: &str = "self_hash";
pub const PACKAGE_HASH: &str = "package_hash";
pub const BUSD_HASH: &str = "busd_hash";
pub const WBNB_HASH: &str = "wbnb_hash";
pub const SBNB_HASH: &str = "sbnb_hash";
pub const WISE_HASH: &str = "wise_hash";
pub const DECIMALS: &str = "decimals";
pub const YODAS_PER_WISE: &str = "yodas_per_wise";
pub const ROUTER_HASH: &str = "router_hash";
pub const PATH: &str = "path";
pub const LATEST_BUSD_EQUIVALENT: &str = "latest_busd_equivalent";
pub const CONSTANTS_DICT: &str = "constants_dict";

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

pub fn busd_hash() -> Key {
    get_key(BUSD_HASH).unwrap_or_revert()
}
pub fn set_busd_hash(hash: Key) {
    set_key(BUSD_HASH, hash);
}

pub fn sbnb_hash() -> Key {
    get_key(SBNB_HASH).unwrap_or_revert()
}
pub fn set_sbnb_hash(hash: Key) {
    set_key(SBNB_HASH, hash);
}

pub fn wbnb_hash() -> Key {
    get_key(WBNB_HASH).unwrap_or_revert()
}
pub fn set_wbnb_hash(hash: Key) {
    set_key(WBNB_HASH, hash);
}

pub fn wise_hash() -> Key {
    get_key(WISE_HASH).unwrap_or_revert()
}
pub fn set_wise_hash(hash: Key) {
    set_key(WISE_HASH, hash);
}

pub fn decimals() -> U256 {
    get_key(DECIMALS).unwrap_or_revert()
}
pub fn set_decimals(decimals: U256) {
    set_key(DECIMALS, decimals);
    set_key(YODAS_PER_WISE, U256::from(10).pow(decimals));
}

pub fn yodas_per_wise() -> U256 {
    get_key(YODAS_PER_WISE).unwrap_or_revert()
}

pub fn latest_busd_equivalent() -> U256 {
    get_key(LATEST_BUSD_EQUIVALENT).unwrap_or_revert()
}
pub fn set_latest_busd_equivalent(latest_busd_equivalent: U256) {
    set_key(LATEST_BUSD_EQUIVALENT, latest_busd_equivalent);
}

pub fn router_hash() -> U256 {
    get_key(ROUTER_HASH).unwrap_or_revert()
}
pub fn set_router_hash(hash: Key) {
    set_key(ROUTER_HASH, hash);
}

pub struct Constants {
    dict: Dict,
}

impl Constants {
    pub fn instance() -> Constants {
        Constants {
            dict: Dict::instance(CONSTANTS_DICT),
        }
    }

    pub fn init() {
        Dict::init(CONSTANTS_DICT)
    }

    pub fn get(&self, key: &str) -> U256 {
        let result: U256 = self.dict.get(&key).unwrap_or_default();
        result
    }

    // value is the json string representation of the 'Scrapes' structure
    // key should the string representation of the Key and Vec<u16> concatinated
    pub fn set(&self, key: &str, value: U256) {
        self.dict.set(&key, value);
    }
}
