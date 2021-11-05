#![no_main]
#![no_std]

extern crate alloc;
use alloc::{collections::BTreeSet, format, vec};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints,
    Group, Key, Parameter, RuntimeArgs, URef, U256, CLType, CLValue,
};
use contract_utils::{ContractContext, OnChainContractStorage};
use wisetoken::config::*;
use wisetoken::{self, WiseToken};

#[derive(Default)]
struct WiseTokenStruct(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for WiseTokenStruct 
{
    fn storage(&self) -> &OnChainContractStorage 
    {
        &self.0
    }
}

impl WiseToken<OnChainContractStorage> for WiseTokenStruct{}
impl WiseTokenStruct
{
    fn constructor(&mut self, contract_hash: ContractHash, package_hash: ContractPackageHash, declaration_contract: Key, synthetic_bnb_address: Key, bep20_address: Key) 
    {
        WiseToken::init(self, Key::from(contract_hash), package_hash, declaration_contract, synthetic_bnb_address, bep20_address);
    }
}

#[no_mangle]
// Constructor is used for internal inititializations. Calling it from outside is not allowed.
fn constructor() 
{
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let declaration_contract: Key = runtime::get_named_arg("declaration_contract");
    let synthetic_bnb_address: Key = runtime::get_named_arg("synthetic_bnb_address");
    let bep20_address: Key = runtime::get_named_arg("bep20_address");

    WiseTokenStruct::default().constructor(contract_hash, package_hash, declaration_contract, synthetic_bnb_address, bep20_address);
}

#[no_mangle]
fn set_liquidity_transfomer()
{
    let immutable_transformer: Key = runtime::get_named_arg("immutable_transformer");
    WiseTokenStruct::default().set_liquidity_transfomer(immutable_transformer);
}

#[no_mangle]
fn set_busd()
{
    let equalizer_address: Key = runtime::get_named_arg("equalizer_address");
    WiseTokenStruct::default().set_busd(equalizer_address);
}

#[no_mangle]
fn renounce_keeper()
{
    WiseTokenStruct::default().renounce_keeper();
}

#[no_mangle]
fn mint_supply()
{
    let investor_address: Key = runtime::get_named_arg("investor_address");
    let amount: U256 = runtime::get_named_arg("amount");
    WiseTokenStruct::default().mint_supply(investor_address, amount);
}


#[no_mangle]
fn create_stake_with_bnb()
{
    let lock_days: Key = runtime::get_named_arg("lock_days");
    let referrer: Key = runtime::get_named_arg("referrer");

    (U256, ) = WiseTokenStruct::default().create_stake_with_bnb(lock_days, referrer);
}

fn get_entry_points() -> EntryPoints 
{
    let mut entry_points = EntryPoints::new();

    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("declaration_contract", CLType::Key),
            Parameter::new("synthetic_bnb_address", CLType::Key),
            Parameter::new("bep20_address", CLType::Key),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_liquidity_transfomer",
        vec![
            Parameter::new("immutable_transformer", CLType::Key)
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_busd",
        vec![
            Parameter::new("equalizer_address", CLType::Key)
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "renounce_keeper",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "mint_supply",
        vec![
            Parameter::new("investor_address", CLType::Key),
            Parameter::new("amount", CLType::U256)
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));


    entry_points.add_entry_point(EntryPoint::new(
        "create_stake_with_bnb",
        vec![
            Parameter::new("lock_days", CLType::U64),
            Parameter::new("referrer", CLType::Key),
        ],
        CLType::Key,
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

    let declaration_contract: Key = runtime::get_named_arg("declaration_contract");
    let synthetic_bnb_address: Key = runtime::get_named_arg("synthetic_bnb_address");
    let bep20_address: Key = runtime::get_named_arg("bep20_address");

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "contract_hash" => contract_hash,
        "package_hash" => package_hash,
        "declaration_contract" => declaration_contract,
        "synthetic_bnb_address" => synthetic_bnb_address,
        "bep20_address" => bep20_address
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