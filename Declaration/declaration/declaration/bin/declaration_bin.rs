#![no_main]
#![no_std]

extern crate alloc;
use alloc::{boxed::Box, collections::BTreeSet, format, vec, string::String};

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
use declaration::{self, Declaration};

#[derive(Default)]
struct DeclarationStruct(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for DeclarationStruct 
{
    fn storage(&self) -> &OnChainContractStorage 
    {
        &self.0
    }
}

impl Declaration<OnChainContractStorage> for DeclarationStruct{}
impl DeclarationStruct
{
    fn constructor(&mut self, contract_hash: ContractHash, package_hash: ContractPackageHash, uniswap_router: Key, factory: Key, pair: Key, liquidity_guard: Key, synthetic_bnb: Key) 
    {
        let launch_time: U256 = U256::from(1619395200);             // set launch time as per the requirments.
        Declaration::init(self, Key::from(contract_hash), package_hash, uniswap_router, factory, pair, liquidity_guard, synthetic_bnb, launch_time);
    }
}

#[no_mangle]
/// Constructor is used for internal inititializations. Calling it from outside is not allowed.
fn constructor() 
{
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let uniswap_router: Key = runtime::get_named_arg("uniswap_router");
    let factory: Key = runtime::get_named_arg("factory");
    let pair: Key = runtime::get_named_arg("pair");
    let liquidity_guard: Key = runtime::get_named_arg("liquidity_guard");
    let synthetic_bnb: Key = runtime::get_named_arg("synthetic_bnb");

    DeclarationStruct::default().constructor(contract_hash, package_hash, uniswap_router, factory, pair, liquidity_guard, synthetic_bnb);
}

#[no_mangle]
fn launch_time()
{
    let launch_time: U256 = DeclarationStruct::default().launch_time();
    runtime::ret(CLValue::from_t(launch_time).unwrap_or_revert());
}

#[no_mangle]
fn get_stake_count()
{
    let staker: Key = runtime::get_named_arg("staker");
    let stake_count: U256 = DeclarationStruct::default().get_stake_count(staker);
    runtime::ret(CLValue::from_t(stake_count).unwrap_or_revert());
}

#[no_mangle]
fn set_stake_count()
{
    let staker: Key = runtime::get_named_arg("staker");
    let value: U256 = runtime::get_named_arg("value");

    let _: () = DeclarationStruct::default().set_stake_count(staker, value);
}

#[no_mangle]
fn get_referral_count()
{
    let referrer: Key = runtime::get_named_arg("referrer");
    let referrer_count: U256 = DeclarationStruct::default().get_referral_count(referrer);
    runtime::ret(CLValue::from_t(referrer_count).unwrap_or_revert());
}

#[no_mangle]
fn set_referral_count()
{
    let referrer: Key = runtime::get_named_arg("referrer");
    let value: U256 = runtime::get_named_arg("value");

    let _: () = DeclarationStruct::default().set_referral_count(referrer, value);
}

#[no_mangle]
fn get_liquidity_stake_count()
{
    let staker: Key = runtime::get_named_arg("staker");
    let liquidity_stake_count: U256 = DeclarationStruct::default().get_liquidity_stake_count(staker);
    runtime::ret(CLValue::from_t(liquidity_stake_count).unwrap_or_revert());
}

#[no_mangle]
fn set_liquidity_stake_count()
{
    let staker: Key = runtime::get_named_arg("staker");
    let value: U256 = runtime::get_named_arg("value");
    let _: () = DeclarationStruct::default().set_liquidity_stake_count(staker, value);
}


#[no_mangle]
fn get_stuct_from_key()
{
    let key: String = runtime::get_named_arg("key");
    let struct_name: String = runtime::get_named_arg("struct_name");

    let ret: String = DeclarationStruct::default().get_stuct_from_key(key, struct_name);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

fn get_entry_points() -> EntryPoints 
{
    let mut entry_points = EntryPoints::new();

    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("uniswap_router", Key::cl_type()),
            Parameter::new("factory", Key::cl_type()),
            Parameter::new("pair", Key::cl_type()),
            Parameter::new("liquidity_guard", Key::cl_type()),
            Parameter::new("synthetic_bnb", Key::cl_type())
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "launch_time",
        vec![],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_stake_count",
        vec![
            Parameter::new("staker", Key::cl_type())
        ],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_stake_count",
        vec![
            Parameter::new("staker", Key::cl_type()),
            Parameter::new("value", CLType::U256)
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_referral_count",
        vec![
            Parameter::new("referrer", Key::cl_type())
        ],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_referral_count",
        vec![
            Parameter::new("referrer", Key::cl_type()),
            Parameter::new("value", CLType::U256)
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_liquidity_stake_count",
        vec![
            Parameter::new("staker", Key::cl_type())
        ],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_liquidity_stake_count",
        vec![
            Parameter::new("staker", Key::cl_type()),
            Parameter::new("value", CLType::U256)
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_struct_from_key",
        vec![
            Parameter::new("key", CLType::String),
            Parameter::new("struct_name", CLType::String),
        ],
        CLType::String,
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

    let uniswap_router: Key = runtime::get_named_arg("uniswap_router");
    let factory: Key = runtime::get_named_arg("factory");
    let pair: Key = runtime::get_named_arg("pair");
    let liquidity_guard: Key = runtime::get_named_arg("liquidity_guard");
    let synthetic_bnb: Key = runtime::get_named_arg("synthetic_bnb");

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "contract_hash" => contract_hash,
        "package_hash" => package_hash,
        "uniswap_router" => uniswap_router,
        "factory" => factory,
        "pair" => pair,
        "liquidity_guard" => liquidity_guard,
        "synthetic_bnb" => synthetic_bnb
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
