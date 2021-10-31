#![no_main]
#![no_std]

extern crate alloc;
use alloc::{collections::BTreeSet, format, vec, string::String};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, CLTyped, CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints,
    Group, Key, Parameter, RuntimeArgs, URef, U256, CLValue
};
use contract_utils::{ContractContext, OnChainContractStorage};
use globals::{self, Globals};

#[derive(Default)]
struct GlobalsContract(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for GlobalsContract 
{
    fn storage(&self) -> &OnChainContractStorage 
    {
        &self.0
    }
}

impl Globals<OnChainContractStorage> for GlobalsContract{}
impl GlobalsContract
{
    fn constructor(&mut self, contract_hash: ContractHash, package_hash: ContractPackageHash) 
    {
        Globals::init(self, Key::from(contract_hash), package_hash);
    }
}

#[no_mangle]
/// Constructor is used for internal inititializations. Calling it from outside is not allowed.
fn constructor() 
{
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    GlobalsContract::default().constructor(contract_hash, package_hash);
}

#[no_mangle]
/// Increase Globals
/// 
/// Parameters-> _staked:U256, _shares:U256, _rshares:U256
fn increase_globals()
{
    let _staked: U256 = runtime::get_named_arg("_staked");
    let _shares: U256 = runtime::get_named_arg("_shares");
    let _rshares: U256 = runtime::get_named_arg("_rshares");
    GlobalsContract::default().increase_globals(_staked, _shares, _rshares);
}

#[no_mangle]
/// Decrease Globals
/// 
/// Parameters-> _staked:U256, _shares:U256, _rshares:U256
fn decrease_globals()
{
    let _staked: U256 = runtime::get_named_arg("_staked");
    let _shares: U256 = runtime::get_named_arg("_shares");
    let _rshares: U256 = runtime::get_named_arg("_rshares");
    GlobalsContract::default().decrease_globals(_staked, _shares, _rshares);
}

#[no_mangle]
fn set_globals()
{
    let field: String = runtime::get_named_arg("field");
    let value: U256 = runtime::get_named_arg("value");
    GlobalsContract::default().set_globals(field, value);
}

#[no_mangle]
fn get_globals()
{
    let field: String = runtime::get_named_arg("field");
    let ret: U256 = GlobalsContract::default().get_globals(field);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}



fn get_entry_points() -> EntryPoints 
{
    let mut entry_points = EntryPoints::new();

    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type())
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "increase_globals",
        vec![
            Parameter::new("_staked", U256::cl_type()),
            Parameter::new("_shares", U256::cl_type()),
            Parameter::new("_rshares", U256::cl_type())
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "decrease_globals",
        vec![
            Parameter::new("_staked", U256::cl_type()),
            Parameter::new("_shares", U256::cl_type()),
            Parameter::new("_rshares", U256::cl_type())
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_globals",
        vec![
            Parameter::new("field", String::cl_type()),
            Parameter::new("value", U256::cl_type())
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_globals",
        vec![
            Parameter::new("field", String::cl_type()),
        ],
        CLType::U256,
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

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "contract_hash" => contract_hash,
        "package_hash" => package_hash
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
