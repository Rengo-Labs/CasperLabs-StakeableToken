#![no_main]
#![no_std]

extern crate alloc;
use alloc::{boxed::Box, collections::BTreeSet, format, vec, vec::Vec};

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
    fn constructor(&mut self, contract_hash: ContractHash, package_hash: ContractPackageHash, declaration_contract: Key, globals_contract: Key, synthetic_bnb_address: Key, bep20_address: Key, router_address: Key, staking_token_address: Key, timing_address: Key) 
    {
        WiseToken::init(self, Key::from(contract_hash), package_hash, declaration_contract, globals_contract, synthetic_bnb_address, bep20_address, router_address, staking_token_address, timing_address);
    }
}

#[no_mangle]
// Constructor is used for internal inititializations. Calling it from outside is not allowed.
fn constructor() 
{
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let declaration_contract: Key = runtime::get_named_arg("declaration_contract");
    let globals_contract: Key = runtime::get_named_arg("globals_address");
    let synthetic_bnb_address: Key = runtime::get_named_arg("synthetic_bnb_address");
    let bep20_address: Key = runtime::get_named_arg("bep20_address");
    let router_address: Key = runtime::get_named_arg("router_address");
    let staking_token_address: Key = runtime::get_named_arg("staking_token_address");
    let timing_address: Key = runtime::get_named_arg("timing_address");

    WiseTokenStruct::default().constructor(contract_hash, package_hash, declaration_contract, globals_contract, synthetic_bnb_address, bep20_address, router_address, staking_token_address, timing_address);
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
    let lock_days: u64 = runtime::get_named_arg("lock_days");
    let referrer: Key = runtime::get_named_arg("referrer");
    let amount: U256 = runtime::get_named_arg("amount");
    let purse: URef = runtime::get_named_arg("purse");

    let (stake_id, start_day, referrer_id):(Vec<u32>, U256 ,Vec<u32>) = WiseTokenStruct::default().create_stake_with_bnb(lock_days, referrer, amount, purse);
    runtime::ret(CLValue::from_t((stake_id, start_day, referrer_id)).unwrap_or_revert());
}

#[no_mangle]
fn create_stake_with_token()
{
    let token_address: Key = runtime::get_named_arg("token_address");
    let token_amount: U256 = runtime::get_named_arg("token_amount");
    let lock_days: u64 = runtime::get_named_arg("lock_days");
    let referrer: Key = runtime::get_named_arg("referrer");

    let (stake_id, start_day, referrer_id):(Vec<u32>, U256 ,Vec<u32>) = WiseTokenStruct::default().create_stake_with_token(token_address, token_amount, lock_days, referrer);
    runtime::ret(CLValue::from_t((stake_id, start_day, referrer_id)).unwrap_or_revert());
}

#[no_mangle]
fn get_pair_address()
{
    let ret: Key = WiseTokenStruct::default().get_pair_address();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_total_staked()
{
    let ret: U256 = WiseTokenStruct::default().get_total_staked();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_liquidity_transformer()
{
    let ret: Key = WiseTokenStruct::default().get_liquidity_transformer();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert()); 
}

#[no_mangle]
fn get_synthetic_token_address()
{
    let ret: Key = WiseTokenStruct::default().get_synthetic_token_address();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert()); 
}

#[no_mangle]
fn extend_lt_auction()
{
    WiseTokenStruct::default().extend_lt_auction();
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
            Parameter::new("globals_address", CLType::Key),
            Parameter::new("synthetic_bnb_address", CLType::Key),
            Parameter::new("bep20_address", CLType::Key),
            Parameter::new("router_address", CLType::Key),
            Parameter::new("staking_token_address", CLType::Key),
            Parameter::new("timing_address", CLType::Key)
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
            Parameter::new("amount", CLType::U256),
            Parameter::new("purse", CLType::URef)
        ],
        CLType::Tuple3([Box::new(CLType::List(Box::new(CLType::U32))), Box::new(CLType::U256), Box::new(CLType::List(Box::new(CLType::U32)))]),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "create_stake_with_token",
        vec![
            Parameter::new("token_address", CLType::Key),               // IBEP20 Token
            Parameter::new("token_amount", CLType::U256),
            Parameter::new("lock_days", CLType::U64),
            Parameter::new("referrer", CLType::Key)
        ],
        CLType::Tuple3([Box::new(CLType::List(Box::new(CLType::U32))), Box::new(CLType::U256), Box::new(CLType::List(Box::new(CLType::U32)))]),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_pair_address",
        vec![],
        CLType::Key,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_total_staked",
        vec![],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_liquidity_transformer",
        vec![],
        CLType::Key,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_synthetic_token_address",
        vec![],
        CLType::Key,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "extend_lt_auction",
        vec![],
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

    let declaration_contract: Key = runtime::get_named_arg("declaration_contract");
    let globals_contract: Key = runtime::get_named_arg("globals_contract");
    let synthetic_bnb_address: Key = runtime::get_named_arg("synthetic_bnb_address");
    let bep20_address: Key = runtime::get_named_arg("bep20_address");
    let router_address: Key = runtime::get_named_arg("router_address");
    let staking_token_address: Key = runtime::get_named_arg("staking_token_address");
    let timing_address: Key = runtime::get_named_arg("timing_address");

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "contract_hash" => contract_hash,
        "package_hash" => package_hash,
        "declaration_contract" => declaration_contract,
        "globals_address" => globals_contract,
        "synthetic_bnb_address" => synthetic_bnb_address,
        "bep20_address" => bep20_address,
        "router_address" => router_address,
        "staking_token_address" => staking_token_address,
        "timing_address" => timing_address
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