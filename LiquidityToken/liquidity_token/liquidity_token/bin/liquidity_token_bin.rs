#![no_main]
#![no_std]

extern crate alloc;
use alloc::{boxed::Box, collections::BTreeSet, format, string::String, vec, vec::Vec};
// use std::boxed::Box;

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
use liquidity_token::{self, LiquidityToken};

#[derive(Default)]
struct LiquidityTokenStruct(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for LiquidityTokenStruct {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl LiquidityToken<OnChainContractStorage> for LiquidityTokenStruct {}
impl LiquidityTokenStruct {
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
        snapshot_contract_hash: Key,
    ) {
        LiquidityToken::init(
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
            snapshot_contract_hash,
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
    let snapshot_hash: Key = runtime::get_named_arg("snapshot");

    LiquidityTokenStruct::default().constructor(
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
        snapshot_hash,
    );
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
            Parameter::new("snapshot", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "create_liquidity_stake",
        vec![Parameter::new("liquidity_tokens", U256::cl_type())],
        CLType::List(Box::new(u32::cl_type())),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "end_liquidity_stake",
        vec![Parameter::new("id", CLType::List(Box::new(u32::cl_type())))],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "check_liquidity_stake_by_id",
        vec![Parameter::new("staker",  Key::cl_type())],
        CLType::String,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points
}

#[no_mangle]
fn end_liquidity_stake() {
    let liquidity_stake_id: Vec<u32> = runtime::get_named_arg("id");

    let ret = LiquidityTokenStruct::default()._end_liquidity_stake(liquidity_stake_id);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn check_liquidity_stake_by_id() {
    let staker: Key = runtime::get_named_arg("staker");
    let liquidity_stake_id: Vec<u32> = runtime::get_named_arg("id");

    let liquidity_stake_string: String =
        LiquidityTokenStruct::default()._check_liquidity_stake_by_id(staker, liquidity_stake_id);

    runtime::ret(CLValue::from_t(liquidity_stake_string).unwrap_or_revert());
}

#[no_mangle]
fn create_liquidity_stake() {
    let liquidity_tokens: U256 = runtime::get_named_arg("liquidity_tokens");

    // VERIFY how to work with Vec<> as return types
    let liquidity_stake_id: Vec<u32> =
        LiquidityTokenStruct::default()._create_liquidity_stake(liquidity_tokens);
    runtime::ret(CLValue::from_t(liquidity_stake_id).unwrap_or_revert());
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
    let snapshot_contract_hash: Key = runtime::get_named_arg("snapshot");
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
        "guard"=>guard_contract_hash,
        "snapshot"=>snapshot_contract_hash
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
