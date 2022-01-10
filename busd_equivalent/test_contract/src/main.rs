#![no_std]
#![no_main]
// #![feature(default_alloc_error_handler)]
// #[cfg(not(target_arch = "wasm32"))]
// compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;
use alloc::{boxed::Box, collections::BTreeSet, format, string::String, vec};

// use core::panic::PanicInfo;

use casper_contract::{
    contract_api::{account, runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints,
    Group, Key, Parameter, RuntimeArgs, URef, U256, U512,
};
// use contract_utils::{get_key, set_key};
pub mod utils;
use utils::*;

// ================================== Test Endpoints ================================== //
#[no_mangle]
fn set_key_by_name() {
    let name: String = runtime::get_named_arg("name");
    let key: Key = runtime::get_named_arg("key");

    set_key(&name, key);
}

#[no_mangle]
fn get_busd_equivalent() {
    let busd_equivalent: ContractHash = get_key("busd_equivalent");

    let ret: U256 =
        runtime::call_contract(busd_equivalent, "get_busd_equivalent", runtime_args! {});
    set_key("get_busd_equivalent_result", ret);
}

#[no_mangle]
fn update_busd_equivalent() {
    let busd_equivalent: ContractHash = get_key(&"busd_equivalent");

    let () = runtime::call_contract(busd_equivalent, "update_busd_equivalent", runtime_args! {});
}

#[no_mangle]
fn add_liquidity() {
    let token_a: Key = runtime::get_named_arg("token_a");
    let token_b: Key = runtime::get_named_arg("token_b");
    let amount_a_desired: U256 = runtime::get_named_arg("amount_a_desired");
    let amount_b_desired: U256 = runtime::get_named_arg("amount_b_desired");
    let amount_a_min: U256 = runtime::get_named_arg("amount_a_min");
    let amount_b_min: U256 = runtime::get_named_arg("amount_b_min");
    let to: Key = runtime::get_named_arg("to");
    let deadline: U256 = runtime::get_named_arg("deadline");
    let pair: Option<Key> = runtime::get_named_arg("pair");
    let router_address: Key = runtime::get_named_arg("router_hash");
    let router_address: ContractHash = _create_hash_from_key(router_address);
    let router_package_hash: ContractPackageHash =
        runtime::call_contract(router_address, "package_hash", runtime_args! {});
    // approve the router to spend tokens
    let _: () = runtime::call_contract(
        ContractHash::from(token_a.into_hash().unwrap_or_revert()),
        "approve",
        runtime_args! {
            "spender" => Key::from(router_package_hash),
            "amount" => amount_a_desired
        },
    );

    let _: () = runtime::call_contract(
        ContractHash::from(token_b.into_hash().unwrap_or_revert()),
        "approve",
        runtime_args! {
            "spender" => Key::from(router_package_hash),
            "amount" => amount_b_desired
        },
    );

    let args: RuntimeArgs = runtime_args! {
        "token_a" => token_a,
        "token_b" => token_b,
        "amount_a_desired" => amount_a_desired,
        "amount_b_desired" => amount_b_desired,
        "amount_a_min" => amount_a_min,
        "amount_b_min" => amount_b_min,
        "to" => to,
        "deadline" => deadline,
        "pair" => pair
    };

    let (amount_a, amount_b, liquidity): (U256, U256, U256) =
        runtime::call_contract(router_address, "add_liquidity", args);
    // set_key("add_liqudity_result", (amount_a, amount_b, liquidity));
}
#[no_mangle]
fn add_liquidity_cspr() {
    let router_address: Key = runtime::get_named_arg("router");
    let router_address: ContractHash = _create_hash_from_key(router_address);
    let self_hash: Key = runtime::get_named_arg("self_hash");
    let self_hash: ContractHash = ContractHash::from(self_hash.into_hash().unwrap_or_revert());

    let token: Key = runtime::get_named_arg("token");
    let amount_token_desired: U256 = runtime::get_named_arg("amount_token_desired");
    let amount_cspr_desired: U256 = runtime::get_named_arg("amount_cspr_desired");
    let amount_token_min: U256 = runtime::get_named_arg("amount_token_min");
    let amount_cspr_min: U256 = runtime::get_named_arg("amount_cspr_min");
    let to: Key = runtime::get_named_arg("to");
    let deadline: U256 = runtime::get_named_arg("deadline");
    let pair: Option<Key> = runtime::get_named_arg("pair");

    let router_package_hash: ContractPackageHash =
        runtime::call_contract(router_address, "package_hash", runtime_args! {});

    // Approve contract
    let _: () = runtime::call_contract(
        ContractHash::from(token.into_hash().unwrap_or_revert()),
        "approve",
        runtime_args! {
            "spender" => Key::from(router_package_hash),
            "amount" => amount_token_desired
        },
    );

    // create dummy contract purse and send some cspr to it
    let self_purse: URef = system::create_purse(); // create contract's purse
    let caller_purse: URef = account::get_main_purse();

    // transfer dummy cspr to self_purse.
    // Since system::transfer_from_purse_to_purse works with contracts having type 'Contract' and since this method has type 'Session' thus creating a sperate entry point for transfering funds between purses
    let _: () = runtime::call_contract(
        self_hash,
        "transfer_cspr",
        runtime_args! {
            "src_purse" => caller_purse,
            "dest_purse" => self_purse,
            "amount" => U512::from(amount_cspr_desired.as_u32())
        },
    );

    let args: RuntimeArgs = runtime_args! {
        "token" => token,
        "amount_token_desired" => amount_token_desired,
        "amount_cspr_desired" => amount_cspr_desired,
        "amount_token_min" => amount_token_min,
        "amount_cspr_min" => amount_cspr_min,
        "to" => to,
        "deadline" => deadline,
        "pair" => pair,
        "purse" => self_purse
    };

    let (amount_token, amount_cspr, liquidity): (U256, U256, U256) =
        runtime::call_contract(router_address, "add_liquidity_cspr", args);
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
            Parameter::new("busd_equivalent", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_busd_equivalent",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_key_by_name",
        vec![
            Parameter::new("name", String::cl_type()),
            Parameter::new("key", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "update_busd_equivalent",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "add_liquidity",
        vec![
            Parameter::new("token_a", CLType::Key),
            Parameter::new("token_b", CLType::Key),
            Parameter::new("amount_a_desired", CLType::U256),
            Parameter::new("amount_b_desired", CLType::U256),
            Parameter::new("amount_a_min", CLType::U256),
            Parameter::new("amount_b_min", CLType::U256),
            Parameter::new("to", CLType::Key),
            Parameter::new("deadline", CLType::U256),
            Parameter::new("pair", CLType::Option(Box::new(CLType::Key))),
            Parameter::new("router_hash", CLType::Key),
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
    let busd_equivalent: Key = runtime::get_named_arg("busd_equivalent");
    set_key(
        "busd_equivalent",
        ContractHash::from(busd_equivalent.into_hash().unwrap_or_default()),
    );
    set_key("contract_hash", contract_hash);
    set_key("package_hash", package_hash);
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
    let busd_equivalent: Key = runtime::get_named_arg("busd_equivalent");

    // Get parameters and pass it to the constructors
    // Prepare constructor args
    let constructor_args = runtime_args! {
        "contract_hash" => contract_hash,
        "package_hash" => package_hash,
        "busd_equivalent"=>busd_equivalent
        // TRANSFER_HELPER_HASH_RUNTIME_ARG_NAME=>transfer_helper
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
