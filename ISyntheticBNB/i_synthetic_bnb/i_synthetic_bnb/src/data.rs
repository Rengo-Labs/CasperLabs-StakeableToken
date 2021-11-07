use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{contracts::ContractPackageHash, Key};
use contract_utils::{get_key, set_key};
extern crate alloc;
// keys for global k-v items
pub const SELF_HASH: &str = "self_hash";
pub const PACKAGE_HASH: &str = "package_hash";
pub const SBNB_HASH: &str = "sbnb_hash";
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

pub fn sbnb_hash() -> Key {
    get_key(SBNB_HASH).unwrap_or_revert()
}
pub fn set_sbnb_hash(hash: Key) {
    set_key(SBNB_HASH, hash);
}
