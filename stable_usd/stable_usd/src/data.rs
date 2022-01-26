use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{contracts::ContractPackageHash, Key, U256};
use contract_utils::{get_key, set_key};
extern crate alloc;
use alloc::{vec, vec::Vec};
use wise_token_utils::commons::key_names::*;

pub fn self_hash() -> Key {
    get_key(SELF_CONTRACT_HASH).unwrap_or_revert()
}
pub fn set_self_hash(hash: Key) {
    set_key(SELF_CONTRACT_HASH, hash);
}

pub fn package_hash() -> ContractPackageHash {
    get_key(SELF_PACKAGE_HASH).unwrap_or_revert()
}
pub fn set_package_hash(hash: ContractPackageHash) {
    set_key(SELF_PACKAGE_HASH, hash);
}

pub fn busd_hash() -> Key {
    get_key(BUSD_CONTRACT_HASH).unwrap_or_revert()
}
pub fn set_busd_hash(hash: Key) {
    set_key(BUSD_CONTRACT_HASH, hash);
}

pub fn scspr_hash() -> Key {
    get_key(SCSPR_CONTRACT_HASH).unwrap_or_revert()
}
pub fn set_scspr_hash(hash: Key) {
    set_key(SCSPR_CONTRACT_HASH, hash);
}

pub fn wcspr_hash() -> Key {
    get_key(WCSPR_CONTRACT_HASH).unwrap_or_revert()
}
pub fn set_wcspr_hash(hash: Key) {
    set_key(WCSPR_CONTRACT_HASH, hash);
}

pub fn wise_hash() -> Key {
    get_key(WISE_TOKEN_CONTRACT_HASH).unwrap_or_revert()
}
pub fn set_wise_hash(hash: Key) {
    set_key(WISE_TOKEN_CONTRACT_HASH, hash);
}

pub fn decimals() -> U256 {
    get_key(DECIMALS).unwrap_or_default()
}
pub fn set_decimals(decimals: U256) {
    set_key(DECIMALS, decimals);
    set_key(STABLE_USD_YODAS_PER_WISE, U256::from(10).pow(decimals));
}

pub fn yodas_per_wise() -> U256 {
    get_key(STABLE_USD_YODAS_PER_WISE).unwrap_or_default()
}

pub fn latest_stable_usd() -> U256 {
    get_key(STABLE_USD_LATEST_STABLE_USD).unwrap_or_default()
}
pub fn set_latest_stable_usd(latest_stable_usd: U256) {
    set_key(STABLE_USD_LATEST_STABLE_USD, latest_stable_usd);
}

pub fn router_hash() -> Key {
    get_key(ROUTER_CONTRACT_HASH).unwrap_or_revert()
}
pub fn set_router_hash(hash: Key) {
    set_key(ROUTER_CONTRACT_HASH, hash);
}

pub fn set_path(key1: Key, key2: Key, key3: Key, key4: Key) {
    let _path = vec![key1, key2, key3, key4];
    set_key(PATH, _path);
}

pub fn get_path() -> Vec<Key> {
    get_key(PATH).unwrap_or_revert()
}

// pub struct Constants {
//     dict: Dict,
// }

// impl Constants {
//     pub fn instance() -> Constants {
//         Constants {
//             dict: Dict::instance(CONSTANTS_DICT),
//         }
//     }

//     pub fn init() {
//         Dict::init(CONSTANTS_DICT)
//     }

//     pub fn get(&self, key: &str) -> U256 {
//         let result: U256 = self.dict.get(&key).unwrap_or_default();
//         result
//     }

//     // value is the json string representation of the 'Scrapes' structure
//     // key should the string representation of the Key and Vec<u16> concatinated
//     pub fn set(&self, key: &str, value: U256) {
//         self.dict.set(&key, value);
//     }
// }
