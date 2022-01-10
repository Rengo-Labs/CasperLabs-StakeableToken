#![no_main]
#![no_std]

extern crate alloc;
use alloc::{boxed::Box, collections::BTreeSet, format, vec, vec::Vec, string::String};

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
use wisetoken::{self, WiseToken};

use declaration_crate::Declaration;
use staking_token_crate::StakingToken;
use globals_crate::Globals;
use timing_crate::Timing;
use helper_crate::Helper;
use liquidity_token_crate::LiquidityToken;
use referral_token_crate::ReferralToken;
use snapshot_crate::Snapshot;
use bep20_crate::BEP20;

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
impl Declaration<OnChainContractStorage> for WiseTokenStruct{}
impl Globals<OnChainContractStorage> for WiseTokenStruct{}
impl Timing<OnChainContractStorage> for WiseTokenStruct{}
impl StakingToken<OnChainContractStorage> for WiseTokenStruct{}
impl Helper<OnChainContractStorage> for WiseTokenStruct{}
impl LiquidityToken<OnChainContractStorage> for WiseTokenStruct{}
impl ReferralToken<OnChainContractStorage> for WiseTokenStruct{}
impl Snapshot<OnChainContractStorage> for WiseTokenStruct{}
impl BEP20<OnChainContractStorage> for WiseTokenStruct{}

impl WiseTokenStruct
{
    fn constructor(
        &mut self, 
        contract_hash: ContractHash, 
        package_hash: ContractPackageHash,
        synthetic_bnb_address: Key, 
        router_address: Key, 
        launch_time: U256,
        factory_address: Key,
        pair_address: Key,
        liquidity_guard: Key,
        wbnb: Key,
    ) 
    {
        WiseToken::init(
            self, 
            Key::from(contract_hash), 
            package_hash, 
            synthetic_bnb_address,
            router_address, 
            launch_time,
            factory_address,
            pair_address,
            liquidity_guard,
            wbnb
        );
    }
}

#[no_mangle]
// Constructor is used for internal inititializations. Calling it from outside is not allowed.
fn constructor() 
{
    // TODO: Need to make parameters name more consistent, specially for ContractHashes
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let synthetic_bnb_address: Key = runtime::get_named_arg("synthetic_bnb_address");
    let router_address: Key = runtime::get_named_arg("router_address");
    let launch_time: U256 = runtime::get_named_arg("launch_time");
    let factory_address: Key = runtime::get_named_arg("factory_address");
    let pair_address: Key = runtime::get_named_arg("pair_address");
    let liquidity_guard: Key = runtime::get_named_arg("liquidity_guard");
    let wbnb: Key = runtime::get_named_arg("wbnb");

    WiseTokenStruct::default().constructor(
        contract_hash, 
        package_hash, 
        synthetic_bnb_address, 
        router_address, 
        launch_time,
        factory_address,
        pair_address,
        liquidity_guard,
        wbnb
    );
}

#[no_mangle]
fn set_liquidity_transfomer()
{
    let immutable_transformer: Key = runtime::get_named_arg("immutable_transformer");
    let transformer_purse: URef = runtime::get_named_arg("transformer_purse");              // purse of immutable_transformer account

    WiseTokenStruct::default().set_liquidity_transfomer(immutable_transformer, transformer_purse);
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
fn change_keeper()
{
    let keeper: Key = runtime::get_named_arg("keeper");
    WiseTokenStruct::default().change_keeper(keeper);
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

///////////
// BEP20 //
///////////
#[no_mangle]
fn set_sbnb() {
    let sbnb: Key = runtime::get_named_arg("sbnb");
    BEP20::set_sbnb(&WiseTokenStruct::default(), sbnb);
}

#[no_mangle]
fn sbnb_burn() {
    let account: Key = runtime::get_named_arg("account");
    let amount: U256 = runtime::get_named_arg("amount");

    BEP20::sbnb_burn(&WiseTokenStruct::default(), account, amount);
}

#[no_mangle]
fn sbnb_mint() {
    let account: Key = runtime::get_named_arg("account");
    let amount: U256 = runtime::get_named_arg("amount");

    BEP20::sbnb_mint(&WiseTokenStruct::default(), account, amount);
}

#[no_mangle]
fn sbnb_approve() {
    let owner: Key = runtime::get_named_arg("owner");
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");

    BEP20::sbnb_approve(&WiseTokenStruct::default(), owner, spender, amount);
}

#[no_mangle]
fn transfer() {
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    let ret: bool = BEP20::transfer(&WiseTokenStruct::default(), recipient, amount);

    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn transfer_from() {
    let owner: Key = runtime::get_named_arg("owner");
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    let ret: bool = BEP20::transfer_from(&WiseTokenStruct::default(), owner, recipient, amount);

    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn approve() {
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    let ret: bool = BEP20::approve(&WiseTokenStruct::default(), spender, amount);
    
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn mint() {
    let account: Key = runtime::get_named_arg("account");
    let amount: U256 = runtime::get_named_arg("amount");
    BEP20::_mint(&WiseTokenStruct::default(), account, amount);
}

#[no_mangle]
fn burn() {
    let amount: U256 = runtime::get_named_arg("amount");
    BEP20::burn(&WiseTokenStruct::default(), amount);
}

#[no_mangle]
fn balance_of() {
    let owner: Key = runtime::get_named_arg("owner");
    let ret: U256 = BEP20::balance_of(&WiseTokenStruct::default(), owner);

    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn name() {
    let ret: String = BEP20::name(&WiseTokenStruct::default());
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn symbol() {
    let ret: String = BEP20::symbol(&WiseTokenStruct::default());
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn allowance() {
    let owner: Key = runtime::get_named_arg("owner");
    let spender: Key = runtime::get_named_arg("spender");

    
    let ret: U256 = BEP20::allowance(&WiseTokenStruct::default(), owner, spender);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn total_supply() {
    let ret: U256 = BEP20::total_supply(&WiseTokenStruct::default());
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
            Parameter::new("synthetic_bnb_address", CLType::Key),
            Parameter::new("router_address", CLType::Key),
            Parameter::new("launch_time", CLType::U256),
            Parameter::new("factory_address", CLType::Key),
            Parameter::new("pair_address", CLType::Key),
            Parameter::new("liquidity_guard", CLType::Key),
            Parameter::new("wbnb", CLType::Key)
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_liquidity_transfomer",
        vec![
            Parameter::new("immutable_transformer", CLType::Key),
            Parameter::new("transformer_purse", CLType::URef)
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
        "change_keeper",
        vec![
            Parameter::new("keeper", CLType::Key)
        ],
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
            Parameter::new("token_address", CLType::Key),
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


    ///////////
    // BEP20 //
    ///////////
    entry_points.add_entry_point(EntryPoint::new(
        "set_sbnb",
        vec![Parameter::new("sbnb", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "sbnb_burn",
        vec![
            Parameter::new("account", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "sbnb_mint",
        vec![
            Parameter::new("account", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "sbnb_approve",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "name",
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "symbol",
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "total_supply",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer",
        vec![
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "allowance",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("spender", Key::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "approve",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer_from",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "mint",
        vec![
            Parameter::new("account", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "burn",
        vec![Parameter::new("amount", U256::cl_type())],
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

    let synthetic_bnb_address: Key = runtime::get_named_arg("sbnb");
    let router_address: Key = runtime::get_named_arg("router");
    let factory_address: Key = runtime::get_named_arg("factory");
    let pair_address: Key = runtime::get_named_arg("pair");
    let liquidity_guard: Key = runtime::get_named_arg("liquidity_guard");
    let wbnb: Key = runtime::get_named_arg("wbnb");
    let launch_time: U256 = runtime::get_named_arg("launch_time");

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "contract_hash" => contract_hash,
        "package_hash" => package_hash,
        "synthetic_bnb_address" => synthetic_bnb_address,
        "router_address" => router_address,
        "launch_time" => launch_time,
        "factory_address" => factory_address,
        "pair_address" => pair_address,
        "liquidity_guard" => liquidity_guard,
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