use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{contracts::ContractPackageHash, Key};
use contract_utils::{get_key, set_key};
extern crate alloc;

// Keys for global struct
pub const TOTAL_STAKED: &str = "total_staked";
pub const TOTAL_SHARES: &str = "total_shares";
pub const SHARE_PRICE: &str = "share_price";
pub const CURRENT_WISE_DAY: &str = "current_wise_day";
pub const REFERRAL_SHARES: &str = "referral_shares";
pub const LIQUIDITY_SHARES: &str = "liquidity_shares";
// keys for global k-v items
pub const SELF_HASH: &str = "self_hash";
pub const PACKAGE_HASH: &str = "package_hash";
pub const HELPER_HASH: &str = "helper_hash";
pub const TIMING_HASH: &str = "timing_hash";
pub const DECLARATION_HASH: &str = "declaration_hash";
pub const GLOBALS_HASH: &str = "globals_hash";
pub const SBNB_HASH: &str = "sbnb_hash";
pub const PAIR_HASH: &str = "pair_hash";
pub const BEP20_HASH: &str = "pair_hash";
pub const GUARD_HASH: &str = "guard_hash";
pub const SNAPSHOT_HASH: &str = "snapshot_hash";
// Keys for Snapshot structs
pub const SNAPSHOTS_DICT: &str = "snapshots_dicts";
pub const RSNAPSHOTS_DICT: &str = "rsnapshots_dicts";
pub const LSNAPSHOTS_DICT: &str = "lsnapshots_dicts";

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

pub fn helper_hash() -> Key {
    get_key(HELPER_HASH).unwrap_or_revert()
}
pub fn set_helper_hash(hash: Key) {
    set_key(HELPER_HASH, hash);
}

pub fn timing_hash() -> Key {
    get_key(TIMING_HASH).unwrap_or_revert()
}
pub fn set_timing_hash(hash: Key) {
    set_key(TIMING_HASH, hash);
}

pub fn declaration_hash() -> Key {
    get_key(DECLARATION_HASH).unwrap_or_revert()
}
pub fn set_declaration_hash(hash: Key) {
    set_key(DECLARATION_HASH, hash);
}

pub fn globals_hash() -> Key {
    get_key(GLOBALS_HASH).unwrap_or_revert()
}
pub fn set_globals_hash(hash: Key) {
    set_key(GLOBALS_HASH, hash);
}

pub fn sbnb_hash() -> Key {
    get_key(SBNB_HASH).unwrap_or_revert()
}
pub fn set_sbnb_hash(hash: Key) {
    set_key(SBNB_HASH, hash);
}

pub fn pair_hash() -> Key {
    get_key(PAIR_HASH).unwrap_or_revert()
}
pub fn set_pair_hash(hash: Key) {
    set_key(PAIR_HASH, hash);
}

pub fn bep20_hash() -> Key {
    get_key(BEP20_HASH).unwrap_or_revert()
}

pub fn set_bep20_hash(hash: Key) {
    set_key(BEP20_HASH, hash);
}

pub fn guard_hash() -> Key {
    get_key(GUARD_HASH).unwrap_or_revert()
}

pub fn set_guard_hash(hash: Key) {
    set_key(GUARD_HASH, hash);
}

pub fn snapshot_hash() -> Key {
    get_key(SNAPSHOT_HASH).unwrap_or_revert()
}

pub fn set_snapshot_hash(hash: Key) {
    set_key(SNAPSHOT_HASH, hash);
}
