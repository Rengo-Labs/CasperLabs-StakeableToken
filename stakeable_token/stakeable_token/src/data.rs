use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{contracts::ContractPackageHash, Key, URef};
use contract_utils::{get_key, set_key};
use stakeable_token_utils::commons::key_names::*;

pub fn contract_hash() -> Key {
    get_key(SELF_CONTRACT_HASH).unwrap_or_revert()
}
pub fn set_contract_hash(hash: Key) {
    set_key(SELF_CONTRACT_HASH, hash);
}

pub fn package_hash() -> ContractPackageHash {
    get_key(SELF_PACKAGE_HASH).unwrap_or_revert()
}
pub fn set_package_hash(hash: ContractPackageHash) {
    set_key(SELF_PACKAGE_HASH, hash);
}

pub fn liquidity_transformer() -> Key {
    get_key(WISE_TOKEN_LIQUIDITY_TRANSFORMER).unwrap_or_revert()
}
pub fn set_liquidity_transformer(hash: Key) {
    set_key(WISE_TOKEN_LIQUIDITY_TRANSFORMER, hash);
}

pub fn liquidity_transformer_purse() -> URef {
    get_key(WISE_TOKEN_LIQUIDITY_TRANSFORMER_PURSE).unwrap_or_revert()
}
pub fn set_liquidity_transformer_purse(purse: URef) {
    set_key(WISE_TOKEN_LIQUIDITY_TRANSFORMER_PURSE, purse);
}

pub fn transformer_gate_keeper() -> Key {
    get_key(WISE_TOKEN_TRANSFORMER_GATE_KEEPER).unwrap_or_revert()
}
pub fn set_transformer_gate_keeper(hash: Key) {
    set_key(WISE_TOKEN_TRANSFORMER_GATE_KEEPER, hash);
}

pub fn router_hash() -> Key {
    get_key(ROUTER_CONTRACT_HASH).unwrap_or_revert()
}
pub fn set_router_hash(hash: Key) {
    set_key(ROUTER_CONTRACT_HASH, hash);
}
