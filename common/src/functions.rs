use crate::{
    errors::Errors,
    keys::{CONTRACT_HASH, PACKAGE_HASH},
};
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{ContractPackageHash, Key};
use casperlabs_contract_utils::{get_key, set_key};

pub fn zero_address() -> Key {
    Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000")
        .unwrap()
}

pub fn account_zero_address() -> Key {
    Key::from_formatted_str(
        "account-hash-0000000000000000000000000000000000000000000000000000000000000000",
    )
    .unwrap()
}

pub fn block_timestamp() -> u64 {
    runtime::get_blocktime().into()
}

pub fn key_to_hash(key: Key, err: Errors) -> ContractPackageHash {
    key.into_hash().unwrap_or_revert_with(err).into()
}

pub fn set_contract_hash(contract_hash: Key) {
    set_key(CONTRACT_HASH, contract_hash);
}
pub fn contract_hash() -> Key {
    get_key(CONTRACT_HASH).unwrap_or_else(zero_address)
}

pub fn set_package_hash(package_hash: Key) {
    set_key(PACKAGE_HASH, package_hash);
}
pub fn package_hash() -> Key {
    get_key(PACKAGE_HASH).unwrap_or_else(zero_address)
}
