use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{contracts::ContractPackageHash, Key};
use contract_utils::{get_key, set_key};

pub const SELF_HASH: &str = "self_hash";
pub const BEP20_HASH: &str = "bep20_token_hash";
pub const TIMING_HASH: &str = "timing_hash";
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

pub fn bep20_hash() -> Key {
    get_key(BEP20_HASH).unwrap_or_revert()
}
pub fn set_bep20_hash(hash: Key) {
    set_key(BEP20_HASH, hash);
}
