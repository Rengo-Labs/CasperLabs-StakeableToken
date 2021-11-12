#![no_main]
#![no_std]

extern crate alloc;
use alloc::{collections::BTreeSet, format, string::String, vec};

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
use snapshot::{self, Snapshot};

#[derive(Default)]
struct SnapshotStruct(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for SnapshotStruct {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl Snapshot<OnChainContractStorage> for SnapshotStruct {}
impl SnapshotStruct {
    fn constructor(
        &mut self,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
        timing_hash: Key,
        declaration_hash: Key,
        globals_hash: Key,
        helper_contract_hash: Key,
        sbnb_contract_hash: Key,
        pair_contract_hash: Key,
        bep20_contract_hash: Key,
        guard_contract_hash: Key,
    ) {
        Snapshot::init(
            self,
            Key::from(contract_hash),
            package_hash,
            timing_hash,
            declaration_hash,
            globals_hash,
            helper_contract_hash,
            sbnb_contract_hash,
            pair_contract_hash,
            bep20_contract_hash,
            guard_contract_hash,
        );
    }
}

#[no_mangle]
// Constructor is used for internal inititializations. Calling it from outside is not allowed.
fn constructor() {
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let timing_hash: Key = runtime::get_named_arg("timing");
    let declaration_hash: Key = runtime::get_named_arg("declaration");
    let helper_hash: Key = runtime::get_named_arg("helper");
    let globals_hash: Key = runtime::get_named_arg("globals");
    let sbnb_hash: Key = runtime::get_named_arg("sbnb");
    let pair_hash: Key = runtime::get_named_arg("pair");
    let bep20_hash: Key = runtime::get_named_arg("bep20");
    let guard_hash: Key = runtime::get_named_arg("guard");
    SnapshotStruct::default().constructor(
        contract_hash,
        package_hash,
        timing_hash,
        declaration_hash,
        globals_hash,
        helper_hash,
        sbnb_hash,
        pair_hash,
        bep20_hash,
        guard_hash,
    );
}

#[no_mangle]
fn manual_daily_snapshot_point() {
    let update_day: u64 = runtime::get_named_arg("update_day");
    SnapshotStruct::default()._manual_daily_snapshot_point(update_day);
}

#[no_mangle]
fn liquidity_guard_trigger() {
    SnapshotStruct::default()._liquidity_guard_trigger();
}

#[no_mangle]
fn manual_daily_snapshot() {
    SnapshotStruct::default()._manual_daily_snapshot();
}

#[no_mangle]
fn get_struct_from_key() {
    let key: U256 = runtime::get_named_arg("key");
    let struct_name: String = runtime::get_named_arg("struct_name");

    let ret: String = SnapshotStruct::default()._get_struct_from_key(&key, struct_name);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn snapshot_trigger() {
    SnapshotStruct::default()._snapshot_trigger();
}

#[no_mangle]
fn set_struct_from_key() {
    let key: U256 = runtime::get_named_arg("key");
    let struct_name: String = runtime::get_named_arg("struct_name");
    let value: String = runtime::get_named_arg("value");

    SnapshotStruct::default()._set_struct_from_key(&key, value, struct_name);
}
fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("timing", Key::cl_type()),
            Parameter::new("declaration", Key::cl_type()),
            Parameter::new("globals", Key::cl_type()),
            Parameter::new("helper", Key::cl_type()),
            Parameter::new("sbnb", Key::cl_type()),
            Parameter::new("pair", Key::cl_type()),
            Parameter::new("bep20", Key::cl_type()),
            Parameter::new("guard", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "liquidity_guard_trigger",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "manual_daily_snapshot",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "manual_daily_snapshot_point",
        vec![Parameter::new("update_day", CLType::U64)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_struct_from_key",
        vec![
            Parameter::new("key", CLType::U256),
            Parameter::new("struct_name", CLType::String),
        ],
        CLType::String,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_struct_from_key",
        vec![
            Parameter::new("key", CLType::U256),
            Parameter::new("struct_name", CLType::String),
            Parameter::new("value", CLType::String),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "snapshot_trigger",
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

    let timing_contract_hash: Key = runtime::get_named_arg("timing");
    let declaration_contract_hash: Key = runtime::get_named_arg("declaration");
    let globals_contract_hash: Key = runtime::get_named_arg("globals");
    let helper_contract_hash: Key = runtime::get_named_arg("helper");
    let sbnb_contract_hash: Key = runtime::get_named_arg("sbnb");
    let pair_contract_hash: Key = runtime::get_named_arg("pair");
    let bep20_contract_hash: Key = runtime::get_named_arg("bep20");
    let guard_contract_hash: Key = runtime::get_named_arg("guard");

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "contract_hash" => contract_hash,
        "package_hash" => package_hash,
        "timing" => timing_contract_hash,
        "declaration" => declaration_contract_hash,
        "globals"=> globals_contract_hash,
        "helper" => helper_contract_hash,
        "sbnb"=>sbnb_contract_hash,
        "pair"=>pair_contract_hash,
        "bep20"=>bep20_contract_hash,
        "guard"=>guard_contract_hash
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
