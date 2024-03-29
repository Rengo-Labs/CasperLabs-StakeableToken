#![no_main]

extern crate alloc;
use alloc::string::String;
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::ToBytes, runtime_args, ApiError, CLTyped, Key, RuntimeArgs, URef, U256,
};
use common::keys::*;

// Key is the same a destination
fn store<T: CLTyped + ToBytes>(key: &str, value: T) {
    // Store `value` under a new unforgeable reference.
    let value_ref: URef = storage::new_uref(value);

    // Wrap the unforgeable reference in a value of type `Key`.
    let value_key: Key = value_ref.into();

    // Store this key under the name "special_value" in context-local storage.
    runtime::put_key(key, value_key);
}

#[no_mangle]
pub extern "C" fn call() {
    let entrypoint: String = runtime::get_named_arg(ENTRYPOINT);
    let package_hash: Key = runtime::get_named_arg(PACKAGE_HASH);

    match entrypoint.as_str() {
        // Stakeable_Token
        GET_INFLATION => {
            let amount: u32 = runtime::get_named_arg("amount");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                GET_INFLATION,
                runtime_args! {
                    "amount" => amount
                },
            );
            store(GET_INFLATION, ret);
        }
        _ => runtime::revert(ApiError::UnexpectedKeyVariant),
    };
}
