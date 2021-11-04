use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{contracts::{ContractPackageHash}, Key, U256};
use contract_utils::{get_key, set_key, Dict};

pub const SELF_HASH: &str = "self_hash";
pub const PACKAGE_HASH: &str = "package_hash";
pub const LIQUIDITY_TRANSFORMER: &str = "liquidity_transformer";
pub const TRANSFORMER_GATE_KEEPER: &str = "transformer_gate_keeper";
pub const DECLARATION_HASH: &str = "declaration_hash";
pub const BEP_HASH: &str = "bep_hash";

pub fn self_hash() -> Key { get_key(SELF_HASH).unwrap_or_revert()}
pub fn set_self_hash(hash: Key) { set_key(SELF_HASH, hash);}

pub fn package_hash() -> ContractPackageHash { get_key(PACKAGE_HASH).unwrap_or_revert()}
pub fn set_package_hash(hash: ContractPackageHash) { set_key(PACKAGE_HASH, hash);}

pub fn liquidity_transformer() -> Key { get_key(LIQUIDITY_TRANSFORMER).unwrap_or_revert()}
pub fn set_liquidity_transformer(hash: Key) { set_key(LIQUIDITY_TRANSFORMER, hash);}

pub fn transformer_gate_keeper() -> Key { get_key(TRANSFORMER_GATE_KEEPER).unwrap_or_revert()}
pub fn set_transformer_gate_keeper(hash: Key) { set_key(TRANSFORMER_GATE_KEEPER, hash);}

pub fn declaration_hash() -> Key {get_key(DECLARATION_HASH).unwrap_or_revert()}
pub fn set_declaration_hash(hash: Key) { set_key(DECLARATION_HASH, hash);}

pub fn bep20_hash() -> Key {get_key(BEP_HASH).unwrap_or_revert()}
pub fn set_bep20_hash(hash: Key) {set_key(BEP_HASH, hash);}
