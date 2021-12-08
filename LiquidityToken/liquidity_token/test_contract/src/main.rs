#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;
use alloc::{boxed::Box, collections::BTreeSet, format, string::String, vec, vec::Vec};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints,
    Group, Key, Parameter, RuntimeArgs, URef, U256,
};

// pub mod utils;
// use utils::*;
use contract_utils::{get_key, set_key};
pub const LIQUIDITY_TOKEN_KEY_NAME: &str = "liquidity_token";

pub const CHECK_LIQUIDITY_STAKE_BY_ID_RESULT_KEY_NAME: &str =
    "check_liquidity_stake_by_id_key_name";
pub const END_LIQUIDITY_STAKE_RESULT_KEY_NAME: &str = "end_liquidity_stake_key_name";
pub const CREATE_LIQUIDITY_STAKE_RESULT_KEY_NAME: &str = "create_liquidity_stake_key_name";

pub const CHECK_LIQUIDITY_STAKE_BY_ID_ENTRY_POINT_NAME: &str = "check_liquidity_stake_by_id";
pub const END_LIQUIDITY_STAKE_ENTRY_POINT_NAME: &str = "end_liquidity_stake_entry";
pub const CREATE_LIQUIDITY_STAKE_ENTRY_POINT_NAME: &str = "create_liquidity_stake";

pub const STAKER_RUNTIME_ARG_NAME: &str = "staker";
pub const STAKE_ID_RUNTIME_ARG_NAME: &str = "id";
pub const LIQUIDITY_TOKENS_RUNTIME_ARG_NAME: &str = "liquidity_tokens";
// ================================== Test Endpoints ================================== //

#[no_mangle]
fn check_liquidity_stake_by_id() {
    let liquidity: ContractHash = get_key(LIQUIDITY_TOKEN_KEY_NAME).unwrap_or_revert();
    let staker: Key = runtime::get_named_arg(STAKER_RUNTIME_ARG_NAME);
    let id: Vec<u32> = runtime::get_named_arg(STAKE_ID_RUNTIME_ARG_NAME);

    let liquidity_stake_string: String = runtime::call_contract(
        liquidity,
        CHECK_LIQUIDITY_STAKE_BY_ID_ENTRY_POINT_NAME,
        runtime_args! {
            STAKER_RUNTIME_ARG_NAME=>staker,
            STAKE_ID_RUNTIME_ARG_NAME =>id
        },
    );
    set_key(
        &CHECK_LIQUIDITY_STAKE_BY_ID_RESULT_KEY_NAME,
        liquidity_stake_string,
    );
}

#[no_mangle]
fn create_liquidity_stake() {
    let liquidity: ContractHash = get_key(LIQUIDITY_TOKEN_KEY_NAME).unwrap_or_revert();
    let liquidity_tokens: U256 = runtime::get_named_arg(LIQUIDITY_TOKENS_RUNTIME_ARG_NAME);

    let stake_id: Vec<u32> = runtime::call_contract(
        liquidity,
        CREATE_LIQUIDITY_STAKE_ENTRY_POINT_NAME,
        runtime_args! {
            LIQUIDITY_TOKENS_RUNTIME_ARG_NAME=>liquidity_tokens
        },
    );
    set_key(CREATE_LIQUIDITY_STAKE_RESULT_KEY_NAME, stake_id);
}

#[no_mangle]
fn end_liquidity_stake() {
    let liquidity: ContractHash = get_key(LIQUIDITY_TOKEN_KEY_NAME).unwrap_or_revert();
    let id: Vec<u32> = runtime::get_named_arg(STAKE_ID_RUNTIME_ARG_NAME);

    let reward_amount: U256 = runtime::call_contract(
        liquidity,
        END_LIQUIDITY_STAKE_ENTRY_POINT_NAME,
        runtime_args! {
            STAKE_ID_RUNTIME_ARG_NAME=>id
        },
    );

    set_key(END_LIQUIDITY_STAKE_RESULT_KEY_NAME, reward_amount);
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
            Parameter::new("liquidity_token", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        CHECK_LIQUIDITY_STAKE_BY_ID_ENTRY_POINT_NAME,
        vec![
            Parameter::new(
                STAKE_ID_RUNTIME_ARG_NAME,
                CLType::List(Box::new(u32::cl_type())),
            ),
            Parameter::new(STAKER_RUNTIME_ARG_NAME, Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        CREATE_LIQUIDITY_STAKE_ENTRY_POINT_NAME,
        vec![Parameter::new(
            LIQUIDITY_TOKENS_RUNTIME_ARG_NAME,
            U256::cl_type(),
        )],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        END_LIQUIDITY_STAKE_ENTRY_POINT_NAME,
        vec![Parameter::new(
            STAKE_ID_RUNTIME_ARG_NAME,
            CLType::List(Box::new(u32::cl_type())),
        )],
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
    let liquidity_token: Key = runtime::get_named_arg("liquidity_token");
    set_key("liquidity_token", liquidity_token);
    set_key("contract_hash", contract_hash);
    set_key("package_hash", package_hash);
}

// All session code must have a `call` entrypoint.
#[no_mangle]
pub extern "C" fn call() {
    // Build new package with initial a first version of the contract.
    let (package_hash, access_token) = storage::create_contract_package_at_hash();
    let (contract_hash, _): (ContractHash, _) =
        storage::add_contract_version(package_hash, get_entry_points(), Default::default());

    let liquidity_token: Key = runtime::get_named_arg("liquidity_token");
    // Get parameters and pass it to the constructors
    // Prepare constructor args
    let constructor_args = runtime_args! {
        "contract_hash" => contract_hash,
        "package_hash" => package_hash,
        "liquidity_token"=>liquidity_token
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
