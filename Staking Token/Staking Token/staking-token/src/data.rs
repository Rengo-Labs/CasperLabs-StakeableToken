use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::Key;
use contract_utils::{get_key, set_key};

pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const SNAPSHOT_CONTRACT_HASH: &str = "snapshot_contract_hash";
pub const DECLARATION_CONTRACT_HASH: &str = "declaration_contract_hash";
pub const TIMING_CONTRACT_HASH: &str = "timing_contract_hash";
pub const HELPER_CONTRACT_HASH: &str = "helper_contract_hash";
pub const BEP20_CONTRACT_HASH: &str = "bep20_contract_hash";
pub const OWNER: &str = "owner";

pub fn set_hash(contract_hash: Key) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_hash() -> Key {
    get_key(SELF_CONTRACT_HASH).unwrap_or_revert()
}
pub fn set_snapshot_hash(snapshot_hash: Key) {
    set_key(SNAPSHOT_CONTRACT_HASH, snapshot_hash);
}

pub fn get_snapshot_hash() -> Key {
    get_key(SNAPSHOT_CONTRACT_HASH).unwrap_or_revert()
}

pub fn set_declaration_hash(declaration_hash: Key) {
    set_key(DECLARATION_CONTRACT_HASH, declaration_hash);
}

pub fn get_declaration_hash() -> Key {
    get_key(DECLARATION_CONTRACT_HASH).unwrap_or_revert()
}

pub fn set_timing_hash(timing_hash: Key) {
    set_key(TIMING_CONTRACT_HASH, timing_hash);
}

pub fn get_timing_hash() -> Key {
    get_key(TIMING_CONTRACT_HASH).unwrap_or_revert()
}

pub fn set_helper_hash(helper_hash: Key) {
    set_key(HELPER_CONTRACT_HASH, helper_hash);
}

pub fn get_helper_hash() -> Key {
    get_key(HELPER_CONTRACT_HASH).unwrap_or_revert()
}

pub fn set_bep20_hash(bep20_hash: Key) {
    set_key(BEP20_CONTRACT_HASH, bep20_hash);
}

pub fn get_bep20_hash() -> Key {
    get_key(BEP20_CONTRACT_HASH).unwrap_or_revert()
}

pub fn set_owner(owner: Key) {
    set_key(OWNER, owner);
}

pub fn get_owner() -> Key {
    get_key(OWNER).unwrap_or_revert()
}
