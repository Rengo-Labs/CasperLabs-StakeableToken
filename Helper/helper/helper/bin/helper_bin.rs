#![no_main]
#![no_std]

extern crate alloc;
use alloc::{boxed::Box, collections::BTreeSet, format, vec};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints,
    Group, Key, Parameter, RuntimeArgs, URef, U256, CLType, CLValue
};
use contract_utils::{ContractContext, OnChainContractStorage};
use helper::{self, Helper};

#[derive(Default)]
struct HelperStruct(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for HelperStruct 
{
    fn storage(&self) -> &OnChainContractStorage 
    {
        &self.0
    }
}

impl Helper<OnChainContractStorage> for HelperStruct{}
impl HelperStruct
{
    fn constructor(&mut self, contract_hash: ContractHash, package_hash: ContractPackageHash, timing_hash: Key, declaration_hash: Key, globals: Key) 
    {
        Helper::init(self, Key::from(contract_hash), package_hash, timing_hash, declaration_hash, globals);
    }
}

#[no_mangle]
// Constructor is used for internal inititializations. Calling it from outside is not allowed.
fn constructor() 
{
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let timing_hash: Key = runtime::get_named_arg("timing");
    let declaration_hash: Key = runtime::get_named_arg("declaration");
    let globals: Key = runtime::get_named_arg("globals");

    HelperStruct::default().constructor(contract_hash, package_hash, timing_hash, declaration_hash, globals);
}

fn get_entry_points() -> EntryPoints 
{
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("timing", Key::cl_type()),
            Parameter::new("declaration", Key::cl_type()),
            Parameter::new("globals", Key::cl_type())
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "stakes_pagination",
        vec![
            Parameter::new("_staker", Key::cl_type()),
            Parameter::new("_offset", CLType::U256),
            Parameter::new("_length", CLType::U256),
        ],
        //CLType::List(Box::new(CLType::List(Box::new(CLType::U256)))),                           // list of (lists of u16)
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "referrals_pagination",
        vec![
            Parameter::new("_referrer", Key::cl_type()),
            Parameter::new("_offset", CLType::U256),
            Parameter::new("_length", CLType::U256)
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points
}


// All session code must have a `call` entrypoint.
#[no_mangle]
pub extern "C" fn call() 
{
    let (package_hash, access_token) = storage::create_contract_package_at_hash();
    let (contract_hash, _) : (ContractHash, _) =
        storage::add_contract_version(package_hash, get_entry_points(), Default::default());

    let timing_contract_hash: Key = runtime::get_named_arg("timing");
    let declaration_contract_hash: Key = runtime::get_named_arg("declaration");
    let globals_hash: Key = runtime::get_named_arg("globals");

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "contract_hash" => contract_hash,
        "package_hash" => package_hash,
        "timing" => timing_contract_hash,
        "declaration" => declaration_contract_hash,
        "globals" => globals_hash
    };

    // Add the constructor group to the package hash with a single URef.
    let constructor_access: URef =
    storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
        .unwrap_or_revert()
        .pop()
        .unwrap_or_revert();

    // Call the constructor entry point
    let _: () = runtime::call_versioned_contract(package_hash, None, "constructor", constructor_args);

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
