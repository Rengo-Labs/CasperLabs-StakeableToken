use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{contracts::{ContractPackageHash}, Key, U256};
use contract_utils::{get_key, set_key, Dict};

pub const SELF_HASH: &str = "self_hash";
pub const PACKAGE_HASH: &str = "package_hash";
pub const DECLARATION_CONTRACT_HASH: &str = "declaration_contract";
pub const LAUNCH_TIME: &str = "launch_time";

pub fn self_hash() -> Key { get_key(SELF_HASH).unwrap_or_revert()}
pub fn set_self_hash(hash: Key) { set_key(SELF_HASH, hash);}

pub fn package_hash() -> ContractPackageHash { get_key(PACKAGE_HASH).unwrap_or_revert()}
pub fn set_package_hash(hash: ContractPackageHash) { set_key(PACKAGE_HASH, hash);}

pub fn declaration_contract_hash() -> Key { get_key(DECLARATION_CONTRACT_HASH).unwrap_or_revert()}
pub fn set_declaration_contract_hash(hash: Key) { set_key(DECLARATION_CONTRACT_HASH, hash);}

pub fn launch_time() -> U256 {get_key(LAUNCH_TIME).unwrap_or_default()}
pub fn set_launch_time(launch_time: U256) { set_key(LAUNCH_TIME, launch_time);}