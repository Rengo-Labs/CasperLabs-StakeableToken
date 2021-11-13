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
    fn constructor(&mut self, contract_hash: ContractHash, package_hash: ContractPackageHash, launch_time: U256, uniswap_router: Key, factory: Key, pair: Key, liquidity_guard: Key, synthetic_bnb: Key, wbnb: Key) 
    {
        Declaration::init(self, Key::from(contract_hash), package_hash, launch_time, uniswap_router, factory, pair, liquidity_guard, synthetic_bnb, wbnb);
    }
}

#[no_mangle]
/// Constructor is used for internal inititializations. Calling it from outside is not allowed.
fn constructor() 
{
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let launch_time: U256 = runtime::get_named_arg("launch_time");
    let uniswap_router: Key = runtime::get_named_arg("uniswap_router");
    let factory: Key = runtime::get_named_arg("factory");
    let pair: Key = runtime::get_named_arg("pair");
    let liquidity_guard: Key = runtime::get_named_arg("liquidity_guard");
    let synthetic_bnb: Key = runtime::get_named_arg("synthetic_bnb");
    let wbnb: Key = runtime::get_named_arg("wbnb");
    
    DeclarationStruct::default().constructor(contract_hash, package_hash, launch_time, uniswap_router, factory, pair, liquidity_guard, synthetic_bnb, wbnb);
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

    DeclarationStruct::default().set_stake_count(staker, value);
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

    DeclarationStruct::default().set_referral_count(referrer, value);
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
    DeclarationStruct::default().set_liquidity_stake_count(staker, value);
}


#[no_mangle]
fn get_struct_from_key()
{
    let key: String = runtime::get_named_arg("key");
    let struct_name: String = runtime::get_named_arg("struct_name");

    let ret: String = DeclarationStruct::default().get_struct_from_key(key, struct_name);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn set_struct_from_key()
{
    let key: String = runtime::get_named_arg("key");
    let value: String = runtime::get_named_arg("value");
    let struct_name: String = runtime::get_named_arg("struct_name");

    DeclarationStruct::default().set_struct_from_key(key, value, struct_name);
}

#[no_mangle]
fn set_referral_shares_to_end()
{
    let key: U256 = runtime::get_named_arg("key");
    let value: U256 = runtime::get_named_arg("value");

    DeclarationStruct::default().set_referral_shares_to_end(key, value);
}

#[no_mangle]
fn get_referral_shares_to_end()
{
    let key: U256 = runtime::get_named_arg("key");

    let ret: U256 = DeclarationStruct::default().get_referral_shares_to_end(key);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn set_scheduled_to_end()
{
    let key: U256 = runtime::get_named_arg("key");
    let value: U256 = runtime::get_named_arg("value");

    DeclarationStruct::default().set_scheduled_to_end(key, value);
}

#[no_mangle]
fn get_scheduled_to_end()
{
    let key: U256 = runtime::get_named_arg("key");

    let ret: U256 = DeclarationStruct::default().get_scheduled_to_end(key);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn set_total_penalties()
{
    let key: U256 = runtime::get_named_arg("key");
    let value: U256 = runtime::get_named_arg("value");

    DeclarationStruct::default().set_total_penalties(key, value);
}

#[no_mangle]
fn get_total_penalties()
{
    let key: U256 = runtime::get_named_arg("key");

    let ret: U256 = DeclarationStruct::default().get_total_penalties(key);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_declaration_constants()
{
    let ret: String = DeclarationStruct::default().get_declaration_constants();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn set_inflation_rate()
{
    let value: U256 = runtime::get_named_arg("value");
    DeclarationStruct::default().set_inflation_rate(value);
}

#[no_mangle]
fn get_inflation_rate()
{
    let ret: U256  = DeclarationStruct::default().get_inflation_rate();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn set_liquidity_rate()
{
    let value: U256 = runtime::get_named_arg("value");
    DeclarationStruct::default().set_liquidity_rate(value);
}

#[no_mangle]
fn get_liquidity_rate()
{
    let ret: U256  = DeclarationStruct::default().get_liquidity_rate();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn set_liquidity_guard_status()
{
    let value: bool = runtime::get_named_arg("value");
    DeclarationStruct::default().set_liquidity_guard_status(value);
}

#[no_mangle]
fn get_liquidity_guard_status()
{
    let ret: bool  = DeclarationStruct::default().get_liquidity_guard_status();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn set_sbnb()
{
    let value: Key = runtime::get_named_arg("sbnb");
    DeclarationStruct::default().set_sbnb(value);
}

#[no_mangle]
fn get_sbnb()
{
    let ret: Key = DeclarationStruct::default().get_sbnb();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_wbnb()
{
    let ret: Key = DeclarationStruct::default().get_wbnb();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn set_busd_eq()
{
    let value: Key = runtime::get_named_arg("busd_eq");
    DeclarationStruct::default().set_busd_eq(value);
}

#[no_mangle]
fn get_busd_eq()
{
    let ret: Key = DeclarationStruct::default().get_busd_eq();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_pancake_pair()
{
    let ret: Key = DeclarationStruct::default().get_pancake_pair();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn set_lt_balance()
{
    let value: U256 = runtime::get_named_arg("value");
    DeclarationStruct::default().set_lt_balance(value);
}

#[no_mangle]
fn get_lt_balance()
{
    let ret: U256 = DeclarationStruct::default().get_lt_balance();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_launchtime()
{
    let ret: U256 = DeclarationStruct::default().get_launchtime();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn set_launchtime()
{
    let value: U256 = runtime::get_named_arg("value");
    DeclarationStruct::default().set_launchtime(value);
}

#[no_mangle]
fn set_scrapes()
{
    let key: String = runtime::get_named_arg("key");
    let value: U256 = runtime::get_named_arg("value");

    DeclarationStruct::default().set_scrapes(key, value);
}

#[no_mangle]
fn get_scrapes()
{
    let key: String = runtime::get_named_arg("key");

    let ret: U256 = DeclarationStruct::default().get_scrapes(key);
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
            Parameter::new("launch_time", CLType::U256),
            Parameter::new("uniswap_router", Key::cl_type()),
            Parameter::new("factory", Key::cl_type()),
            Parameter::new("pair", Key::cl_type()),
            Parameter::new("liquidity_guard", Key::cl_type()),
            Parameter::new("synthetic_bnb", Key::cl_type()),
            Parameter::new("wbnb", Key::cl_type())
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
            Parameter::new("staker", CLType::Key)
        ],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_stake_count",
        vec![
            Parameter::new("staker", CLType::Key),
            Parameter::new("value", CLType::U256)
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_referral_count",
        vec![
            Parameter::new("referrer", CLType::Key)
        ],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_referral_count",
        vec![
            Parameter::new("referrer", CLType::Key),
            Parameter::new("value", CLType::U256)
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_liquidity_stake_count",
        vec![
            Parameter::new("staker", CLType::Key)
        ],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_liquidity_stake_count",
        vec![
            Parameter::new("staker", CLType::Key),
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

    entry_points.add_entry_point(EntryPoint::new(
        "set_struct_from_key",
        vec![
            Parameter::new("key", CLType::String),
            Parameter::new("value", CLType::String),
            Parameter::new("struct_name", CLType::String),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_referral_shares_to_end",
        vec![
            Parameter::new("key", CLType::U256),
            Parameter::new("value", CLType::U256)
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    
    entry_points.add_entry_point(EntryPoint::new(
        "get_referral_shares_to_end",
        vec![
            Parameter::new("key", CLType::U256)
        ],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_scheduled_to_end",
        vec![
            Parameter::new("key", CLType::U256),
            Parameter::new("value", CLType::U256)
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    
    entry_points.add_entry_point(EntryPoint::new(
        "get_scheduled_to_end",
        vec![
            Parameter::new("key", CLType::U256)
        ],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    
    entry_points.add_entry_point(EntryPoint::new(
        "set_total_penalties",
        vec![
            Parameter::new("key", CLType::U256),
            Parameter::new("value", CLType::U256)
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    
    entry_points.add_entry_point(EntryPoint::new(
        "get_total_penalties",
        vec![
            Parameter::new("key", CLType::U256)
        ],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_inflation_rate",
        vec![
            Parameter::new("value", CLType::U256)
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_inflation_rate",
        vec![],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_liquidity_rate",
        vec![
            Parameter::new("value", CLType::U256)
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_liquidity_rate",
        vec![],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    
    entry_points.add_entry_point(EntryPoint::new(
        "get_liquidity_guard_status",
        vec![],
        CLType::Bool,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_liquidity_guard_status",
        vec![
            Parameter::new("value", CLType::Bool)
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_declaration_constants",
        vec![],
        CLType::String,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_sbnb",
        vec![
            Parameter::new("sbnb", CLType::Key)
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_sbnb",
        vec![],
        CLType::Key,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_wbnb",
        vec![],
        CLType::Key,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_busd_eq",
        vec![
            Parameter::new("busd_eq", CLType::Key)
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_busd_eq",
        vec![],
        CLType::Key,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_pancake_pair",
        vec![],
        CLType::Key,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_launchtime",
        vec![],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_launchtime",
        vec![
            Parameter::new("value", CLType::U256)
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_lt_balance",
        vec![],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_lt_balance",
        vec![
            Parameter::new("value", CLType::U256)
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_scrapes",
        vec![
            Parameter::new("key", CLType::String),
            Parameter::new("value", CLType::U256),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_scrapes",
        vec![
            Parameter::new("key", CLType::String)
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

    let launch_time: U256 = runtime::get_named_arg("launch_time");                      // in epoch milliseconds
    let uniswap_router: Key = runtime::get_named_arg("uniswap_router");
    let factory: Key = runtime::get_named_arg("factory");
    let pair: Key = runtime::get_named_arg("pair");
    let liquidity_guard: Key = runtime::get_named_arg("liquidity_guard");
    let synthetic_bnb: Key = runtime::get_named_arg("synthetic_bnb");
    let wbnb: Key = runtime::get_named_arg("wbnb");

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "contract_hash" => contract_hash,
        "package_hash" => package_hash,
        "launch_time" => launch_time,
        "uniswap_router" => uniswap_router,
        "factory" => factory,
        "pair" => pair,
        "liquidity_guard" => liquidity_guard,
        "synthetic_bnb" => synthetic_bnb,
        "wbnb" => wbnb
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
