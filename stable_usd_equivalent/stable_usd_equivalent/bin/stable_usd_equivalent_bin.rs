#![no_main]
#![no_std]

extern crate alloc;
use alloc::{collections::BTreeSet, format, vec};

use stable_usd_equivalent::{self, StableUSD};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, CLType, CLTyped, CLValue, EntryPoint, EntryPointAccess, EntryPointType,
    EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};
use contract_utils::{ContractContext, OnChainContractStorage};

#[derive(Default)]
struct StableUSDStruct(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for StableUSDStruct {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl StableUSD<OnChainContractStorage> for StableUSDStruct {}
impl StableUSDStruct {
    fn constructor(
        &mut self,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
        stakeable: Key,
        scspr: Key,
        wcspr: Key,
        stable_usd: Key,
        router: Key,
    ) {
        StableUSD::init(
            self,
            Key::from(contract_hash),
            package_hash,
            stakeable,
            scspr,
            wcspr,
            stable_usd,
            router,
        );
    }
}

#[no_mangle]
fn get_stable_usd_equivalent() {
    let ret: U256 = StableUSDStruct::default().get_stable_usd_equivalent();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn update_stable_usd_equivalent() {
    StableUSDStruct::default().update_stable_usd_equivalent();
}

#[no_mangle]
// Constructor is used for internal inititializations. Calling it from outside is not allowed.
fn constructor() {
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let stakeable: Key = runtime::get_named_arg("stakeable");
    let scspr: Key = runtime::get_named_arg("scspr");
    let wcspr: Key = runtime::get_named_arg("wcspr");
    let stable_usd: Key = runtime::get_named_arg("stable_usd");
    let router: Key = runtime::get_named_arg("router");

    StableUSDStruct::default().constructor(
        contract_hash,
        package_hash,
        stakeable,
        scspr,
        wcspr,
        stable_usd,
        router,
    );
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("stakeable", Key::cl_type()),
            Parameter::new("scspr", Key::cl_type()),
            Parameter::new("wcspr", Key::cl_type()),
            Parameter::new("stable_usd", Key::cl_type()),
            Parameter::new("router", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_stable_usd_equivalent",
        vec![],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "update_stable_usd_equivalent",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points
}

// All session code must have a `call` entrypoint.
#[no_mangle]
pub extern "C" fn call() {
    let (package_hash, access_token) = storage::create_contract_package_at_hash();
    let (contract_hash, _): (ContractHash, _) =
        storage::add_contract_version(package_hash, get_entry_points(), Default::default());

    let stakeable: Key = runtime::get_named_arg("stakeable");
    let scspr: Key = runtime::get_named_arg("scspr");
    let wcspr: Key = runtime::get_named_arg("wcspr");
    let stable_usd: Key = runtime::get_named_arg("stable_usd");
    let router: Key = runtime::get_named_arg("router");

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "contract_hash" => contract_hash,
        "package_hash" => package_hash,
        "stakeable" => stakeable,
        "scspr" => scspr,
        "wcspr" => wcspr,
        "stable_usd" => stable_usd,
        "router"=>router,
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
