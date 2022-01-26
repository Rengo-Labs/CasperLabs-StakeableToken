#![no_main]
#![no_std]

extern crate alloc;
use alloc::{boxed::Box, collections::BTreeSet, format, string::String, vec, vec::Vec};

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
use wisetoken::{self, WiseToken};

use declaration_crate::Declaration;
use erc20_crate::{self, ERC20};
use globals_crate::Globals;
use helper_crate::Helper;
use liquidity_token_crate::LiquidityToken;
use referral_token_crate::ReferralToken;
use snapshot_crate::Snapshot;
use staking_token_crate::StakingToken;
use timing_crate::Timing;

#[derive(Default)]
struct WiseTokenStruct(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for WiseTokenStruct {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl WiseToken<OnChainContractStorage> for WiseTokenStruct {}
impl Declaration<OnChainContractStorage> for WiseTokenStruct {}
impl Globals<OnChainContractStorage> for WiseTokenStruct {}
impl Timing<OnChainContractStorage> for WiseTokenStruct {}
impl StakingToken<OnChainContractStorage> for WiseTokenStruct {}
impl Helper<OnChainContractStorage> for WiseTokenStruct {}
impl LiquidityToken<OnChainContractStorage> for WiseTokenStruct {}
impl ReferralToken<OnChainContractStorage> for WiseTokenStruct {}
impl Snapshot<OnChainContractStorage> for WiseTokenStruct {}
impl ERC20<OnChainContractStorage> for WiseTokenStruct {}

impl WiseTokenStruct {
    fn constructor(
        &mut self,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
        synthetic_cspr_address: Key,
        router_address: Key,
        launch_time: U256,
        factory_address: Key,
        pair_address: Key,
        liquidity_guard: Key,
        wcspr: Key,
        domain_separator: String,
        permit_type_hash: String,
    ) {
        WiseToken::init(
            self,
            Key::from(contract_hash),
            package_hash,
            synthetic_cspr_address,
            router_address,
            launch_time,
            factory_address,
            pair_address,
            liquidity_guard,
            wcspr,
            domain_separator,
            permit_type_hash,
        );
    }
}

#[no_mangle]
// Constructor is used for internal inititializations. Calling it from outside is not allowed.
fn constructor() {
    // TODO: Need to make parameters name more consistent, specially for ContractHashes
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let synthetic_cspr_address: Key = runtime::get_named_arg("synthetic_cspr_address");
    let router_address: Key = runtime::get_named_arg("router_address");
    let launch_time: U256 = runtime::get_named_arg("launch_time");
    let factory_address: Key = runtime::get_named_arg("factory_address");
    let pair_address: Key = runtime::get_named_arg("pair_address");
    let liquidity_guard: Key = runtime::get_named_arg("liquidity_guard");
    let wcspr: Key = runtime::get_named_arg("wcspr");
    let domain_separator: String = runtime::get_named_arg("domain_separator");
    let permit_type_hash: String = runtime::get_named_arg("permit_type_hash");

    WiseTokenStruct::default().constructor(
        contract_hash,
        package_hash,
        synthetic_cspr_address,
        router_address,
        launch_time,
        factory_address,
        pair_address,
        liquidity_guard,
        wcspr,
        domain_separator,
        permit_type_hash,
    );
}

#[no_mangle]
fn set_liquidity_transfomer() {
    let immutable_transformer: Key = runtime::get_named_arg("immutable_transformer");
    let transformer_purse: URef = runtime::get_named_arg("transformer_purse"); // purse of immutable_transformer account

    WiseTokenStruct::default().set_liquidity_transfomer(immutable_transformer, transformer_purse);
}

#[no_mangle]
fn set_stable_usd() {
    let equalizer_address: Key = runtime::get_named_arg("equalizer_address");
    WiseToken::set_stable_usd(&WiseTokenStruct::default(), equalizer_address);
}

#[no_mangle]
fn renounce_keeper() {
    WiseTokenStruct::default().renounce_keeper();
}

#[no_mangle]
fn change_keeper() {
    let keeper: Key = runtime::get_named_arg("keeper");
    WiseTokenStruct::default().change_keeper(keeper);
}

#[no_mangle]
fn mint_supply() {
    let investor_address: Key = runtime::get_named_arg("investor_address");
    let amount: U256 = runtime::get_named_arg("amount");
    WiseTokenStruct::default().mint_supply(investor_address, amount);
}

#[no_mangle]
fn create_stake_with_cspr() {
    let lock_days: u64 = runtime::get_named_arg("lock_days");
    let referrer: Key = runtime::get_named_arg("referrer");
    let amount: U256 = runtime::get_named_arg("amount");
    let purse: URef = runtime::get_named_arg("purse");

    let (stake_id, start_day, referrer_id): (Vec<u32>, U256, Vec<u32>) =
        WiseTokenStruct::default().create_stake_with_cspr(lock_days, referrer, amount, purse);
    runtime::ret(CLValue::from_t((stake_id, start_day, referrer_id)).unwrap_or_revert());
}

#[no_mangle]
fn create_stake_with_token() {
    let token_address: Key = runtime::get_named_arg("token_address");
    let token_amount: U256 = runtime::get_named_arg("token_amount");
    let lock_days: u64 = runtime::get_named_arg("lock_days");
    let referrer: Key = runtime::get_named_arg("referrer");

    let (stake_id, start_day, referrer_id): (Vec<u32>, U256, Vec<u32>) = WiseTokenStruct::default()
        .create_stake_with_token(token_address, token_amount, lock_days, referrer);
    runtime::ret(CLValue::from_t((stake_id, start_day, referrer_id)).unwrap_or_revert());
}

#[no_mangle]
fn get_pair_address() {
    let ret: Key = WiseTokenStruct::default().get_pair_address();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_total_staked() {
    let ret: U256 = WiseTokenStruct::default().get_total_staked();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_liquidity_transformer() {
    let ret: Key = WiseTokenStruct::default().get_liquidity_transformer();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_synthetic_token_address() {
    let ret: Key = WiseTokenStruct::default().get_synthetic_token_address();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn extend_lt_auction() {
    WiseTokenStruct::default().extend_lt_auction();
}

/// This function is to transfer tokens against the address that user provided
///
/// # Parameters
///
/// * `recipient` - A Key that holds the account address of the user
///
/// * `amount` - A U256 that holds the amount for transfer
///  

#[no_mangle]
fn transfer() {
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    let ret = ERC20::transfer(&WiseTokenStruct::default(), recipient, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to transfer tokens against the address that has been approved before by owner
///
/// # Parameters
///
/// * `owner` - A Key that holds the account address of the user
///  
/// * `recipient` - A Key that holds the account address of the user
///
/// * `amount` - A U256 that holds the amount for transfer
///
/// **Recommendation:**
///
/// The exploit is mitigated through use of functions that increase/decrease the allowance relative to its current value, such as `increaseAllowance()` and `decreaseAllowance()`.
///
/// Pending community agreement on an ERC standard that would protect against this exploit, we recommend that developers of applications dependent on approve() / transferFrom()
///
/// should keep in mind that they have to set allowance to 0 first and verify if it was used before setting the new value.
///
/// **Note:**  Teams who decide to wait for such a standard should make these
///
/// recommendations to app developers who work with their token contract.

#[no_mangle]
fn transfer_from() {
    let owner: Key = runtime::get_named_arg("owner");
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    let ret = ERC20::transfer_from(&WiseTokenStruct::default(), owner, recipient, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to get meta transaction signer and verify if it is equal
/// to the signer public key or not then call approve.
///
/// # Parameters
///
/// * `public_key` - A string slice that holds the public key of the meta transaction signer,  Subscriber have to get it from running cryptoxide project externally.
///
/// * `signature` - A string slice that holds the signature of the meta transaction,  Subscriber have to get it from running cryptoxide project externally.
///
/// * `owner` - A Key that holds the account address of the owner
///
/// * `spender` - A Key that holds the account address of the spender
///  
/// * `value` - A U256 that holds the value
///  
/// * `deadeline` - A u64 that holds the deadline limit
///

#[no_mangle]
fn permit() {
    let public_key: String = runtime::get_named_arg("public");
    let signature: String = runtime::get_named_arg("signature");
    let owner: Key = runtime::get_named_arg("owner");
    let spender: Key = runtime::get_named_arg("spender");
    let value: U256 = runtime::get_named_arg("value");
    let deadline: u64 = runtime::get_named_arg("deadline");
    WiseTokenStruct::default().permit(public_key, signature, owner, spender, value, deadline);
}

/// This function is to approve tokens against the address that user provided
///
/// # Parameters
///
/// * `spender` - A Key that holds the account address of the user
///
/// * `amount` - A U256 that holds the amount for approve
///
/// **Recommendation:**
///
/// The exploit is mitigated through use of functions that increase/decrease the allowance relative to its current value, such as `increaseAllowance()` and `decreaseAllowance()`.
///
/// Pending community agreement on an ERC standard that would protect against this exploit, we recommend that developers of applications dependent on approve() / transferFrom()
///
/// should keep in mind that they have to set allowance to 0 first and verify if it was used before setting the new value.
///
/// **Note:**  Teams who decide to wait for such a standard should make these
///
/// recommendations to app developers who work with their token contract.

#[no_mangle]
fn approve() {
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    WiseTokenStruct::default().approve(spender, amount);
}

/// This function is to mint token against the address that user provided
///
/// # Parameters
///
/// * `to` - A Key that holds the account address of the user
///
/// * `amount` - A U256 that holds the amount for mint
///

#[no_mangle]
fn mint() {
    let to: Key = runtime::get_named_arg("to");
    let amount: U256 = runtime::get_named_arg("amount");
    WiseTokenStruct::default().mint(to, amount);
}

/// This function is to burn token against the address that user provided
///
/// # Parameters
///
/// * `from` - A Key that holds the account address of the user
///
/// * `amount` - A U256 that holds the amount for burn
///

#[no_mangle]
fn burn() {
    let from: Key = runtime::get_named_arg("from");
    let amount: U256 = runtime::get_named_arg("amount");
    WiseTokenStruct::default().burn(from, amount);
}

/// This function is to return the Balance  of owner against the address that user provided
///
/// # Parameters
///
/// * `owner` - A Key that holds the account address of the user against which user wants to get balance
///

#[no_mangle]
fn balance_of() {
    let owner: Key = runtime::get_named_arg("owner");
    let ret: U256 = WiseTokenStruct::default().balance_of(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to return the Nonce of owner against the address that user provided
///
/// # Parameters
///
/// * `owner` - A Key that holds the account address of the user against which user wants to get nonce
///

#[no_mangle]
fn nonce() {
    let owner: Key = runtime::get_named_arg("owner");
    let ret: U256 = WiseTokenStruct::default().nonce(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to return the Name of contract
///

#[no_mangle]
fn name() {
    let ret: String = WiseTokenStruct::default().name();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to return the Symbol of contract
///

#[no_mangle]
fn symbol() {
    let ret: String = WiseTokenStruct::default().symbol();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to return the Allowance of owner and spender that user provided
///
/// # Parameters
///
/// * `owner` - A Key that holds the account address of the user
///
/// * `spender` - A Key that holds the account address of the user
///

#[no_mangle]
fn allowance() {
    let owner: Key = runtime::get_named_arg("owner");
    let spender: Key = runtime::get_named_arg("spender");
    let ret: U256 = WiseTokenStruct::default().allowance(owner, spender);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to return the Total Supply of the contract
///

#[no_mangle]
fn total_supply() {
    let ret: U256 = WiseTokenStruct::default().total_supply();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to increase the amount of tokens approved for a spender by an owner
///
/// # Parameters
///
/// * `amount` - Number of tokens to increment approval of tokens by for spender
///
/// * `spender` - A Key that holds the account address of the user
///
#[no_mangle]
fn increase_allowance() {
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");

    let ret: Result<(), u32> = WiseTokenStruct::default().increase_allowance(spender, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to decrease the amount of tokens approved for a spender by an owner
///
/// # Parameters
///
/// * `amount` - Number of tokens to decrement approval of tokens by for spender
///
/// * `spender` - A Key that holds the account address of the user
///
#[no_mangle]
fn decrease_allowance() {
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");

    let ret: Result<(), u32> = WiseTokenStruct::default().decrease_allowance(spender, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to fetch a Contract Package Hash
///

#[no_mangle]
fn package_hash() {
    let ret: ContractPackageHash = WiseTokenStruct::default().get_package_hash();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();

    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("synthetic_cspr_address", CLType::Key),
            Parameter::new("router_address", CLType::Key),
            Parameter::new("launch_time", CLType::U256),
            Parameter::new("factory_address", CLType::Key),
            Parameter::new("pair_address", CLType::Key),
            Parameter::new("liquidity_guard", CLType::Key),
            Parameter::new("wcspr", CLType::Key),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_liquidity_transfomer",
        vec![
            Parameter::new("immutable_transformer", CLType::Key),
            Parameter::new("transformer_purse", CLType::URef),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_stable_usd",
        vec![Parameter::new("equalizer_address", CLType::Key)],
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
        vec![Parameter::new("keeper", CLType::Key)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "mint_supply",
        vec![
            Parameter::new("investor_address", CLType::Key),
            Parameter::new("amount", CLType::U256),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "create_stake_with_cspr",
        vec![
            Parameter::new("lock_days", CLType::U64),
            Parameter::new("referrer", CLType::Key),
            Parameter::new("amount", CLType::U256),
            Parameter::new("purse", CLType::URef),
        ],
        CLType::Tuple3([
            Box::new(CLType::List(Box::new(CLType::U32))),
            Box::new(CLType::U256),
            Box::new(CLType::List(Box::new(CLType::U32))),
        ]),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "create_stake_with_token",
        vec![
            Parameter::new("token_address", CLType::Key),
            Parameter::new("token_amount", CLType::U256),
            Parameter::new("lock_days", CLType::U64),
            Parameter::new("referrer", CLType::Key),
        ],
        CLType::Tuple3([
            Box::new(CLType::List(Box::new(CLType::U32))),
            Box::new(CLType::U256),
            Box::new(CLType::List(Box::new(CLType::U32))),
        ]),
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

    entry_points.add_entry_point(EntryPoint::new(
        "transfer",
        vec![
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        CLType::Result {
            ok: Box::new(CLType::Unit),
            err: Box::new(CLType::U32),
        },
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
        CLType::Result {
            ok: Box::new(CLType::Unit),
            err: Box::new(CLType::U32),
        },
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "permit",
        vec![
            Parameter::new("public", String::cl_type()),
            Parameter::new("signature", String::cl_type()),
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("value", U256::cl_type()),
            Parameter::new("deadline", u64::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "approve",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
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
        "nonce",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
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
        "total_supply",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "mint",
        vec![
            Parameter::new("to", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "burn",
        vec![
            Parameter::new("from", Key::cl_type()),
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
        "package_hash",
        vec![],
        ContractPackageHash::cl_type(),
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
        "increase_allowance",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        CLType::Result {
            ok: Box::new(CLType::Unit),
            err: Box::new(CLType::U32),
        },
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "decrease_allowance",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        CLType::Result {
            ok: Box::new(CLType::Unit),
            err: Box::new(CLType::U32),
        },
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

    let (domain_separator, permit_type_hash) = WiseTokenStruct::default()
        .get_permit_type_and_domain_separator("Wise Token", contract_hash);

    let synthetic_cspr_address: Key = runtime::get_named_arg("scspr");
    let router_address: Key = runtime::get_named_arg("router");
    let factory_address: Key = runtime::get_named_arg("factory");
    let pair_address: Key = runtime::get_named_arg("pair");
    let liquidity_guard: Key = runtime::get_named_arg("liquidity_guard");
    let wcspr: Key = runtime::get_named_arg("wcspr");
    let launch_time: U256 = runtime::get_named_arg("launch_time");

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "contract_hash" => contract_hash,
        "package_hash" => package_hash,
        "synthetic_cspr_address" => synthetic_cspr_address,
        "router_address" => router_address,
        "launch_time" => launch_time,
        "factory_address" => factory_address,
        "pair_address" => pair_address,
        "liquidity_guard" => liquidity_guard,
        "wcspr" => wcspr,
        "domain_separator" => domain_separator,
        "permit_type_hash" => permit_type_hash,
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
