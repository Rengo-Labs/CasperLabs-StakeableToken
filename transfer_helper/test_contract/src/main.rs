#![no_std]
#![no_main]
// #![feature(default_alloc_error_handler)]
#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;
use alloc::{collections::BTreeSet, format, string::String, vec};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints,
    Group, Key, Parameter, RuntimeArgs, URef, U256,
};

pub mod constants;
use constants::*;

pub mod utils;
use utils::*;

// ================================== Test Endpoints ================================== //
#[no_mangle]
fn forward_funds() {
    let transfer_helper_hash: ContractHash = get_key(&TRANSFER_HELPER_HASH_RUNTIME_ARG_NAME);
    let token_address: Key = runtime::get_named_arg(TOKEN_ADDRESS_RUNTIME_ARG_NAME);
    let forward_amount: U256 = runtime::get_named_arg(FORWARD_AMOUNT_RUNTIME_ARG_NAME);

    let ret: bool = runtime::call_contract(
        transfer_helper_hash,
        FORWARD_FUNDS_ENTRYPOINT_NAME,
        runtime_args! {
            TOKEN_ADDRESS_RUNTIME_ARG_NAME=>token_address,
            FORWARD_AMOUNT_RUNTIME_ARG_NAME=>forward_amount
        },
    );

    set_key(FORWARD_FUNDS_RESULT, ret);
}

#[no_mangle]
fn set_transfer_helper() {
    let name: String = runtime::get_named_arg("name");
    let key: Key = runtime::get_named_arg("key");

    set_key(&name, ContractHash::from(key.into_hash().unwrap_or_revert()));
}

#[no_mangle]
fn get_transfer_invoker_address() {
    let transfer_helper_hash : ContractHash = get_key(&TRANSFER_HELPER_HASH_KEY_NAME);
    let ret: Key = runtime::call_contract(
        transfer_helper_hash,
        GET_TRANSFER_INVOKER_ADDRESS_ENTRYPOINT_NAME,
        runtime_args! {},
    );
    set_key(GET_TRANSFER_INVOKER_ADDRESS_RESULT, ret);
}

#[no_mangle]
fn transfer() {
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    let erc20 : ContractHash = get_key("erc20");
    let ret: Result<(), u32> = runtime::call_contract(erc20, "transfer", runtime_args!{
        "recipient"=>recipient,
        "amount"=>amount
    });
    set_key("transfer_result", ret);
}

// ================================== Helper functions ============================ //
fn _create_hash_from_key(key: Key) -> ContractHash {
    ContractHash::from(key.into_hash().unwrap_or_default())
}

// ================================ Test Contract Construction =========================== //
fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            // Parameter::new(TRANSFER_HELPER_HASH_RUNTIME_ARG_NAME, Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        FORWARD_FUNDS_ENTRYPOINT_NAME,
        vec![
            Parameter::new(TOKEN_ADDRESS_RUNTIME_ARG_NAME, Key::cl_type()),
            Parameter::new(FORWARD_AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        GET_TRANSFER_INVOKER_ADDRESS_ENTRYPOINT_NAME,
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_transfer_helper",
        vec![
            Parameter::new(NAME_RUNTIME_ARG_NAME, CLType::String),
            Parameter::new(KEY_RUNTIME_ARG_NAME, CLType::Key),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer",
        vec![
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}

#[no_mangle]
fn constructor() {
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    // let transfer_helper: Key = runtime::get_named_arg(TRANSFER_HELPER_HASH_RUNTIME_ARG_NAME);

    set_key("contract_hash", contract_hash);
    set_key("package_hash", package_hash);
    // set_key(TRANSFER_HELPER_HASH_KEY_NAME, transfer_helper);
}

// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {
//     loop {}
// }

// All session code must have a `call` entrypoint.
#[no_mangle]
pub extern "C" fn call() {
    // Build new package with initial a first version of the contract.
    let (package_hash, access_token) = storage::create_contract_package_at_hash();
    let (contract_hash, _): (ContractHash, _) =
        storage::add_contract_version(package_hash, get_entry_points(), Default::default());
    // let transfer_helper: Key = runtime::get_named_arg(TRANSFER_HELPER_HASH_KEY_NAME);

    // Get parameters and pass it to the constructors
    // Prepare constructor args
    let constructor_args = runtime_args! {
        CONTRACT_HASH_RUNTIME_ARG_NAME => contract_hash,
        PACKAGE_HASH_RUNTIME_ARG_NAME => package_hash,
    };

    // Add the constructor group to the package hash with a single URef.
    let constructor_access: URef =
        storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
            .unwrap_or_revert()
            .pop()
            .unwrap_or_revert();

    // Call the constructor entry point
    let _: () =
        runtime::call_versioned_contract(package_hash, None, "constructor", constructor_args);

    // Remove all URefs from the constructor group, so no one can call it for the second time.
    let mut urefs = BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
        .unwrap_or_revert();

    // Store contract in the account's named keys.
    let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");

    runtime::put_key(
        &format!("{}_package_hash", contract_name),
        package_hash.into(),
    );
    runtime::put_key(
        &format!("{}_package_hash_wrapped", contract_name),
        storage::new_uref(package_hash).into(),
    );
    runtime::put_key(
        &format!("{}_contract_hash", contract_name),
        contract_hash.into(),
    );
    runtime::put_key(
        &format!("{}_contract_hash_wrapped", contract_name),
        storage::new_uref(contract_hash).into(),
    );
    runtime::put_key(
        &format!("{}_package_access_token", contract_name),
        access_token.into(),
    );
}
