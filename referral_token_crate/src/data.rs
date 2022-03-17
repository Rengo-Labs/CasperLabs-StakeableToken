use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::Key;
use contract_utils::{get_key, set_key};
use stakeable_token_utils::commons::key_names::*;

pub fn get_hash() -> Key {
    get_key(SELF_CONTRACT_HASH).unwrap_or_revert()
}

pub fn set_owner(owner: Key) {
    set_key(OWNER, owner);
}

pub fn get_owner() -> Key {
    get_key(OWNER).unwrap_or_revert()
}