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
use common::keys::{FORWARD_FUNDS, GET_TRANSFER_INVOKER_ADDRESS};

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
    let entrypoint: String = runtime::get_named_arg("entrypoint");
    let package_hash: Key = runtime::get_named_arg("package_hash");

    match entrypoint.as_str() {
        // Stakeable_Token
        FORWARD_FUNDS => {
            let token_address: Key = runtime::get_named_arg("token_address");
            let forward_amount: U256 = runtime::get_named_arg("forward_amount");
            let ret: bool = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                FORWARD_FUNDS,
                runtime_args! {
                    "token_address" => token_address,
                    "forward_amount" => forward_amount
                },
            );
            store(FORWARD_FUNDS, ret);
        }
        GET_TRANSFER_INVOKER_ADDRESS => {
            let ret: Key = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                GET_TRANSFER_INVOKER_ADDRESS,
                runtime_args! {},
            );
            store(GET_TRANSFER_INVOKER_ADDRESS, ret);
        }
        _ => runtime::revert(ApiError::UnexpectedKeyVariant),
    };
}
