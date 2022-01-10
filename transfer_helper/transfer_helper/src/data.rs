use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{contracts::ContractPackageHash, Key};
use contract_utils::{get_key, set_key};
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

pub fn transfer_invoker() -> Key {
    get_key(TRANSFER_HELPER_TRANSFER_INVOKER).unwrap_or_revert()
}
pub fn set_transfer_invoker(hash: Key) {
    set_key(TRANSFER_HELPER_TRANSFER_INVOKER, hash);
}
