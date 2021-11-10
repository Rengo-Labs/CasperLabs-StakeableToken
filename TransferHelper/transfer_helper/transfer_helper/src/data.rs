use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{contracts::ContractPackageHash, Key};
use contract_utils::{get_key, set_key};

pub const SELF_HASH: &str = "self_hash";
pub const TRANSFER_INVOKER: &str = "transfer_invoker";
pub const PACKAGE_HASH: &str = "package_hash";

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

pub fn transfer_invoker() -> Key {
    get_key(TRANSFER_INVOKER).unwrap_or_revert()
}
pub fn set_transfer_invoker(hash: Key) {
    set_key(TRANSFER_INVOKER, hash);
}
