use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{contracts::ContractPackageHash, Key};
use contract_utils::{get_key, set_key};
extern crate alloc;
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

pub fn scspr_hash() -> Key {
    get_key(SCSPR_CONTRACT_HASH).unwrap_or_revert()
}
pub fn set_scspr_hash(hash: Key) {
    set_key(SCSPR_CONTRACT_HASH, hash);
}

pub fn pair_hash() -> Key {
    get_key(PAIR_CONTRACT_HASH).unwrap_or_revert()
}
pub fn set_pair_hash(hash: Key) {
    set_key(PAIR_CONTRACT_HASH, hash);
}

pub fn guard_hash() -> Key {
    get_key(LIQUIDITY_GUARD_CONTRACT_HASH).unwrap_or_revert()
}

pub fn set_guard_hash(hash: Key) {
    set_key(LIQUIDITY_GUARD_CONTRACT_HASH, hash);
}