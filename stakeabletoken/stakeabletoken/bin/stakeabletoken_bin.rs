#![no_main]
#![no_std]

extern crate alloc;
use alloc::{
    boxed::Box,
    collections::BTreeSet,
    fmt::{Debug, Display},
    format,
    str::FromStr,
    string::{String, ToString},
    vec,
    vec::Vec,
};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::FromBytes,
    bytesrepr::ToBytes,
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, ApiError, CLType, CLTyped, CLValue, EntryPoint, EntryPointAccess, EntryPointType,
    EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U128, U256,
};
use contract_utils::{ContractContext, OnChainContractStorage};
use declaration_crate::Declaration;
use erc20_crate::{self, data, ERC20};
use globals_crate::Globals;
use helper_crate::Helper;
use liquidity_token_crate::LiquidityToken;
use referral_token_crate::ReferralToken;
use snapshot_crate::Snapshot;
use stakeable_token_utils::{
    declaration,
    helpers::{typecast_from_string, typecast_to_string},
    referral_token,
};
use stakeabletoken::{self, StakeableToken};
use staking_token_crate::StakingToken;
use timing_crate::Timing;

#[derive(Default)]
struct StakeableTokenStruct(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for StakeableTokenStruct {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl StakeableToken<OnChainContractStorage> for StakeableTokenStruct {}
impl Declaration<OnChainContractStorage> for StakeableTokenStruct {}
impl Globals<OnChainContractStorage> for StakeableTokenStruct {}
impl Timing<OnChainContractStorage> for StakeableTokenStruct {}
impl StakingToken<OnChainContractStorage> for StakeableTokenStruct {}
impl Helper<OnChainContractStorage> for StakeableTokenStruct {}
impl LiquidityToken<OnChainContractStorage> for StakeableTokenStruct {}
impl ReferralToken<OnChainContractStorage> for StakeableTokenStruct {}
impl Snapshot<OnChainContractStorage> for StakeableTokenStruct {}
impl ERC20<OnChainContractStorage> for StakeableTokenStruct {}

impl StakeableTokenStruct {
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
        StakeableToken::init(
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

    StakeableTokenStruct::default().constructor(
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

// JS Client methods

#[no_mangle]
fn create_stake_with_cspr_Jsclient() {
    let lock_days: u64 = runtime::get_named_arg("lock_days");
    let referrer: Key = runtime::get_named_arg("referrer");
    let amount: U256 = runtime::get_named_arg("amount");
    let purse: URef = runtime::get_named_arg("purse");

    let (_stake_id, _start_day, _referrer_id): (Vec<u32>, U256, Vec<u32>) =
        StakeableTokenStruct::default().create_stake_with_cspr(lock_days, referrer, amount, purse);
}

#[no_mangle]
fn create_stake_with_token_Jsclient() {
    let token_address: Key = runtime::get_named_arg("token_address");
    let token_amount: U256 = runtime::get_named_arg("token_amount");
    let lock_days: u64 = runtime::get_named_arg("lock_days");
    let referrer: Key = runtime::get_named_arg("referrer");

    let (_stake_id, _start_day, _referrer_id): (Vec<u32>, U256, Vec<u32>) =
        StakeableTokenStruct::default().create_stake_with_token(
            token_address,
            token_amount,
            lock_days,
            referrer,
        );
}

#[no_mangle]
fn transfer_Jsclient() {
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    let _ret = ERC20::transfer(&StakeableTokenStruct::default(), recipient, amount);
}

#[no_mangle]
fn transfer_from_Jsclient() {
    let owner: Key = runtime::get_named_arg("owner");
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    let _ret = ERC20::transfer_from(&StakeableTokenStruct::default(), owner, recipient, amount);
}

#[no_mangle]
fn check_referrals_by_id_Jsclient() {
    let _referrer: Key = runtime::get_named_arg("referrer");
    let _referral_id: Vec<String> = runtime::get_named_arg("referral_id");
    let referral_id: Vec<u32> = typecast_from_string(_referral_id);
    let (
        staker,
        stake_id,
        referrer_shares,
        referral_interest,
        is_active_referral,
        is_active_stake,
        is_mature_stake,
        is_ended_stake,
    ): (Key, Vec<u32>, U256, U256, bool, bool, bool, bool) =
        StakeableTokenStruct::default().check_referrals_by_id(_referrer, referral_id);
    let stake_struct = referral_token::structs::StakeInfo {
        staker,
        stake_id,
        referrer_shares,
        referral_interest,
        is_active_referral,
        is_active_stake,
        is_mature_stake,
        is_ended_stake,
    };
    let _ret = stake_struct.clone().into_bytes().unwrap();
}

#[no_mangle]
fn create_stake_Jsclient() {
    let staked_amount: U256 = runtime::get_named_arg("staked_amount");
    let lock_days: u64 = runtime::get_named_arg("lock_days");
    let referrer: Key = runtime::get_named_arg("referrer");
    let (_stake_id, _start_day, _referral_id): (Vec<u32>, U256, Vec<u32>) =
        StakeableTokenStruct::default().create_stake(staked_amount, lock_days, referrer);
}

#[no_mangle]
fn end_stake_Jsclient() {
    let stake_id: Vec<String> = runtime::get_named_arg("stake_id");
    let stake_id: Vec<u32> = typecast_from_string(stake_id);
    let _ret: U256 = StakeableTokenStruct::default().end_stake(stake_id);
}

#[no_mangle]
fn scrape_interest_Jsclient() {
    let stake_id: Vec<String> = runtime::get_named_arg("stake_id");
    let stake_id: Vec<u32> = typecast_from_string(stake_id);
    let scrape_days: u64 = runtime::get_named_arg("scrape_days");
    let (scrape_day, scrape_amount, remaining_days, stakers_penalty, referrer_penalty): (
        U256,
        U256,
        U256,
        U256,
        U256,
    ) = StakeableTokenStruct::default().scrape_interest(stake_id, scrape_days);
    let _ret: Vec<U256> = vec![
        scrape_day,
        scrape_amount,
        remaining_days,
        stakers_penalty,
        referrer_penalty,
    ];
}

#[no_mangle]
fn check_mature_stake_Jsclient() {
    let staker: Key = runtime::get_named_arg("staker");
    let stake_id: Vec<String> = runtime::get_named_arg("stake_id");
    let stake_id: Vec<u32> = typecast_from_string(stake_id);
    let _ret: bool = StakeableTokenStruct::default().check_mature_stake(staker, stake_id);
}

#[no_mangle]
fn check_stake_by_id_Jsclient() {
    let staker: Key = runtime::get_named_arg("staker");
    let stake_id: Vec<String> = runtime::get_named_arg("stake_id");
    let stake_id: Vec<u32> = typecast_from_string(stake_id);

    let (_stake, _penalty_amount, _is_mature): (Vec<u8>, U256, bool) =
        StakeableTokenStruct::default().check_stake_by_id(staker, stake_id);
}

#[no_mangle]
fn create_liquidity_stake_Jsclient() {
    let liquidity_tokens: U256 = runtime::get_named_arg("liquidity_tokens");
    let _liquidity_stake_id: Vec<u32> =
        StakeableTokenStruct::default().create_liquidity_stake(liquidity_tokens);
}

#[no_mangle]
fn end_liquidity_stake_Jsclient() {
    let liquidity_stake_id: Vec<String> = runtime::get_named_arg("liquidity_stake_id");
    let liquidity_stake_id: Vec<u32> = typecast_from_string(liquidity_stake_id);

    let ret: U256 = StakeableTokenStruct::default().end_liquidity_stake(liquidity_stake_id);
}

#[no_mangle]
fn check_liquidity_stake_by_id_Jsclient() {
    let staker: Key = runtime::get_named_arg("staker");
    let liquidity_stake_id: Vec<String> = runtime::get_named_arg("liquidity_stake_id");
    let liquidity_stake_id: Vec<u32> = typecast_from_string(liquidity_stake_id);
    let _ret: Vec<u8> =
        StakeableTokenStruct::default().check_liquidity_stake_by_id(staker, liquidity_stake_id);
}

#[no_mangle]
fn set_liquidity_transfomer() {
    let immutable_transformer: Key = runtime::get_named_arg("immutable_transformer");
    let transformer_purse: URef = runtime::get_named_arg("transformer_purse"); // purse of immutable_transformer account

    StakeableTokenStruct::default()
        .set_liquidity_transfomer(immutable_transformer, transformer_purse);
}

#[no_mangle]
fn set_stable_usd_equivalent() {
    let equalizer_address: Key = runtime::get_named_arg("equalizer_address");
    StakeableToken::set_stable_usd_equivalent(&StakeableTokenStruct::default(), equalizer_address);
}

#[no_mangle]
fn renounce_keeper() {
    StakeableTokenStruct::default().renounce_keeper();
}

#[no_mangle]
fn change_keeper() {
    let keeper: Key = runtime::get_named_arg("keeper");
    StakeableTokenStruct::default().change_keeper(keeper);
}

#[no_mangle]
fn mint_supply() {
    let investor_address: Key = runtime::get_named_arg("investor_address");
    let amount: U256 = runtime::get_named_arg("amount");
    StakeableTokenStruct::default().mint_supply(investor_address, amount);
}

#[no_mangle]
fn create_stake_with_cspr() {
    let lock_days: u64 = runtime::get_named_arg("lock_days");
    let referrer: Key = runtime::get_named_arg("referrer");
    let amount: U256 = runtime::get_named_arg("amount");
    let purse: URef = runtime::get_named_arg("purse");

    let (stake_id, start_day, referrer_id): (Vec<u32>, U256, Vec<u32>) =
        StakeableTokenStruct::default().create_stake_with_cspr(lock_days, referrer, amount, purse);
    runtime::ret(
        CLValue::from_t((
            typecast_to_string(stake_id),
            start_day,
            typecast_to_string(referrer_id),
        ))
        .unwrap_or_revert(),
    );
}

#[no_mangle]
fn create_stake_with_token() {
    let token_address: Key = runtime::get_named_arg("token_address");
    let token_amount: U256 = runtime::get_named_arg("token_amount");
    let lock_days: u64 = runtime::get_named_arg("lock_days");
    let referrer: Key = runtime::get_named_arg("referrer");

    let (stake_id, start_day, referrer_id): (Vec<u32>, U256, Vec<u32>) =
        StakeableTokenStruct::default().create_stake_with_token(
            token_address,
            token_amount,
            lock_days,
            referrer,
        );
    runtime::ret(
        CLValue::from_t((
            typecast_to_string(stake_id),
            start_day,
            typecast_to_string(referrer_id),
        ))
        .unwrap_or_revert(),
    );
}

#[no_mangle]
fn get_pair_address() {
    let ret: Key = StakeableTokenStruct::default().get_pair_address();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_total_staked() {
    let ret: U256 = StakeableTokenStruct::default().get_total_staked();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_liquidity_transformer() {
    let ret: Key = StakeableTokenStruct::default().get_liquidity_transformer();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_synthetic_token_address() {
    let ret: Key = StakeableTokenStruct::default().get_synthetic_token_address();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn extend_lt_auction() {
    StakeableTokenStruct::default().extend_lt_auction();
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
    let ret = ERC20::transfer(&StakeableTokenStruct::default(), recipient, amount);
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
    let ret = ERC20::transfer_from(&StakeableTokenStruct::default(), owner, recipient, amount);
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
    StakeableTokenStruct::default().permit(public_key, signature, owner, spender, value, deadline);
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
    StakeableTokenStruct::default().approve(spender, amount);
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
    StakeableTokenStruct::default().mint(to, amount);
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
    StakeableTokenStruct::default().burn(from, amount);
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
    let ret: U256 = StakeableTokenStruct::default().balance_of(owner);
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
    let ret: U256 = StakeableTokenStruct::default().nonce(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to return the Name of contract
///

#[no_mangle]
fn name() {
    let ret: String = StakeableTokenStruct::default().name();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to return the Symbol of contract
///

#[no_mangle]
fn symbol() {
    let ret: String = StakeableTokenStruct::default().symbol();
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
    let ret: U256 = StakeableTokenStruct::default().allowance(owner, spender);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to return the Total Supply of the contract
///

#[no_mangle]
fn total_supply() {
    let ret: U256 = StakeableTokenStruct::default().total_supply();
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

    let ret: Result<(), u32> = StakeableTokenStruct::default().increase_allowance(spender, amount);
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

    let ret: Result<(), u32> = StakeableTokenStruct::default().decrease_allowance(spender, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to fetch a Contract Package Hash
///

#[no_mangle]
fn package_hash() {
    let ret: ContractPackageHash = StakeableTokenStruct::default().get_package_hash();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn current_stakeable_day() {
    let ret: u64 = StakeableTokenStruct::default().current_stakeable_day();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn liquidity_guard_trigger() {
    StakeableTokenStruct::default().liquidity_guard_trigger();
}

#[no_mangle]
fn manual_daily_snapshot() {
    StakeableTokenStruct::default().manual_daily_snapshot();
}

#[no_mangle]
fn manual_daily_snapshot_point() {
    let update_day: u64 = runtime::get_named_arg("update_day");
    StakeableTokenStruct::default().manual_daily_snapshot_point(update_day);
}

#[no_mangle]
fn get_stable_usd_equivalent() {
    let ret: U256 =
        <StakeableTokenStruct as ReferralToken<OnChainContractStorage>>::get_stable_usd_equivalent(
            &StakeableTokenStruct::default(),
        );
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn referrer_interest() {
    let referral_id: Vec<String> = runtime::get_named_arg("referral_id");
    let referral_id: Vec<u32> = typecast_from_string(referral_id);
    let scrape_days: U256 = runtime::get_named_arg("scrape_days");
    StakeableTokenStruct::default().referrer_interest(referral_id, scrape_days);
}

#[no_mangle]
fn referrer_interest_bulk() {
    let _referral_ids: Vec<Vec<String>> = runtime::get_named_arg("referral_ids");
    let mut referral_ids: Vec<Vec<u32>> = Vec::new();
    for id in _referral_ids {
        referral_ids.push(typecast_from_string(id));
    }
    let scrape_days: Vec<String> = runtime::get_named_arg("scrape_days");
    let scrape_days: Vec<U256> = typecast_from_string(scrape_days); // TODO verify this function works for U256
    StakeableTokenStruct::default().referrer_interest_bulk(referral_ids, scrape_days);
}

#[no_mangle]
fn check_referrals_by_id() {
    let _referrer: Key = runtime::get_named_arg("referrer");
    let referral_id: Vec<String> = runtime::get_named_arg("referral_id");
    let referral_id: Vec<u32> = typecast_from_string(referral_id);

    let (
        staker,
        stake_id,
        referrer_shares,
        referral_interest,
        is_active_referral,
        is_active_stake,
        is_mature_stake,
        is_ended_stake,
    ): (Key, Vec<u32>, U256, U256, bool, bool, bool, bool) =
        StakeableTokenStruct::default().check_referrals_by_id(_referrer, referral_id);
    let ret: Vec<String> = vec![
        staker.to_formatted_string(),
        format!("{:?}", typecast_to_string(stake_id)),
        referrer_shares.to_string(),
        referral_interest.to_string(),
        is_active_referral.to_string(),
        is_active_stake.to_string(),
        is_mature_stake.to_string(),
        is_ended_stake.to_string(),
    ];
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn create_stake_bulk() {
    let staked_amount: Vec<String> = runtime::get_named_arg("staked_amount");
    let staked_amount: Vec<U256> = typecast_from_string(staked_amount);

    let lock_days: Vec<String> = runtime::get_named_arg("lock_days");
    let lock_days: Vec<u64> = typecast_from_string(lock_days);

    let _referrer: Vec<String> = runtime::get_named_arg("referrer");
    let mut referrer: Vec<Key> = Vec::new();
    for key in _referrer.iter() {
        referrer.push(Key::from_formatted_str(key).unwrap());
    }

    StakeableTokenStruct::default().create_stake_bulk(staked_amount, lock_days, referrer);
}

#[no_mangle]
fn create_stake() {
    let staked_amount: U256 = runtime::get_named_arg("staked_amount");
    let lock_days: u64 = runtime::get_named_arg("lock_days");
    let referrer: Key = runtime::get_named_arg("referrer");
    let (stake_id, start_day, referral_id): (Vec<u32>, U256, Vec<u32>) =
        StakeableTokenStruct::default().create_stake(staked_amount, lock_days, referrer);
    runtime::ret(
        CLValue::from_t((
            typecast_to_string(stake_id),
            start_day,
            typecast_to_string(referral_id),
        ))
        .unwrap_or_revert(),
    );
}

#[no_mangle]
fn end_stake() {
    let stake_id: Vec<String> = runtime::get_named_arg("stake_id");
    let stake_id: Vec<u32> = typecast_from_string(stake_id);
    let ret: U256 = StakeableTokenStruct::default().end_stake(stake_id);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn scrape_interest() {
    let stake_id: Vec<String> = runtime::get_named_arg("stake_id");
    let stake_id: Vec<u32> = typecast_from_string(stake_id);
    let scrape_days: u64 = runtime::get_named_arg("scrape_days");
    let (scrape_day, scrape_amount, remaining_days, stakers_penalty, referrer_penalty): (
        U256,
        U256,
        U256,
        U256,
        U256,
    ) = StakeableTokenStruct::default().scrape_interest(stake_id, scrape_days);
    let ret: Vec<String> = vec![
        scrape_day.to_string(),
        scrape_amount.to_string(),
        remaining_days.to_string(),
        stakers_penalty.to_string(),
        referrer_penalty.to_string(),
    ];
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn check_mature_stake() {
    let staker: Key = runtime::get_named_arg("staker");
    let stake_id: Vec<String> = runtime::get_named_arg("stake_id");
    let stake_id: Vec<u32> = typecast_from_string(stake_id);
    let ret: bool = StakeableTokenStruct::default().check_mature_stake(staker, stake_id);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn check_stake_by_id() {
    let staker: Key = runtime::get_named_arg("staker");
    let stake_id: Vec<u32> = runtime::get_named_arg("stake_id");
    let (stake, penalty_amount, is_mature): (Vec<u8>, U256, bool) =
        StakeableTokenStruct::default().check_stake_by_id(staker, stake_id);
    let stake: declaration::structs::Stake =
        declaration::structs::Stake::from_bytes(&stake).unwrap().0;
    let ret: Vec<String> = vec![
        stake.start_day.to_string(),
        stake.lock_days.to_string(),
        stake.final_day.to_string(),
        stake.close_day.to_string(),
        stake.scrape_day.to_string(),
        stake.staked_amount.to_string(),
        stake.stakes_shares.to_string(),
        stake.reward_amount.to_string(),
        penalty_amount.to_string(),
        is_mature.to_string(),
        stake.is_active.to_string(),
    ];
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn create_liquidity_stake() {
    let liquidity_tokens: U256 = runtime::get_named_arg("liquidity_tokens");
    let liquidity_stake_id: Vec<u32> =
        StakeableTokenStruct::default().create_liquidity_stake(liquidity_tokens);
    runtime::ret(CLValue::from_t(typecast_to_string(liquidity_stake_id)).unwrap_or_revert());
}

#[no_mangle]
fn end_liquidity_stake() {
    let liquidity_stake_id: Vec<String> = runtime::get_named_arg("liquidity_stake_id");
    let liquidity_stake_id: Vec<u32> = typecast_from_string(liquidity_stake_id);
    let ret: U256 = StakeableTokenStruct::default().end_liquidity_stake(liquidity_stake_id);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn check_liquidity_stake_by_id() {
    let staker: Key = runtime::get_named_arg("staker");
    let liquidity_stake_id: Vec<String> = runtime::get_named_arg("liquidity_stake_id");
    let liquidity_stake_id: Vec<u32> = typecast_from_string(liquidity_stake_id);
    let _ret: Vec<u8> =
        StakeableTokenStruct::default().check_liquidity_stake_by_id(staker, liquidity_stake_id);
    let l_stake: declaration::structs::LiquidityStake =
        declaration::structs::LiquidityStake::from_bytes(&_ret)
            .unwrap()
            .0;
    let ret: Vec<String> = vec![
        l_stake.start_day.to_string(),
        l_stake.reward_amount.to_string(),
        l_stake.close_day.to_string(),
        l_stake.is_active.to_string(),
    ];
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn generate_id() {
    let x: Key = runtime::get_named_arg("x");
    let y: U256 = runtime::get_named_arg("y");
    let z: u8 = runtime::get_named_arg("z");

    let ret = StakeableTokenStruct::default().generate_id(x, y, z);
    runtime::ret(CLValue::from_t(typecast_to_string(ret)).unwrap_or_revert());
}

#[no_mangle]
fn stakes_pagination() {
    let staker: Key = runtime::get_named_arg("staker");
    let offset: U256 = runtime::get_named_arg("offset");
    let length: U256 = runtime::get_named_arg("length");

    let _ret = StakeableTokenStruct::default().stakes_pagination(staker, offset, length);
    let mut ret: Vec<Vec<String>> = Vec::new();
    for id in _ret {
        ret.push(typecast_to_string(id));
    }
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn referrals_pagination() {
    let staker: Key = runtime::get_named_arg("referrer");
    let offset: U256 = runtime::get_named_arg("offset");
    let length: U256 = runtime::get_named_arg("length");

    let _ret = StakeableTokenStruct::default().referrals_pagination(staker, offset, length);
    let mut ret: Vec<Vec<String>> = Vec::new();
    for id in _ret {
        ret.push(typecast_to_string(id));
    }
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn latest_stake_id() {
    let staker: Key = runtime::get_named_arg("staker");
    let ret = StakeableTokenStruct::default().latest_stake_id(staker);
    runtime::ret(CLValue::from_t(typecast_to_string(ret)).unwrap_or_revert());
}

#[no_mangle]
fn latest_referral_id() {
    let staker: Key = runtime::get_named_arg("staker");
    let ret = StakeableTokenStruct::default().latest_referrer_id(staker);
    runtime::ret(CLValue::from_t(typecast_to_string(ret)).unwrap_or_revert());
}

#[no_mangle]
fn latest_liquidity_stake_id() {
    let staker: Key = runtime::get_named_arg("staker");
    let ret = StakeableTokenStruct::default().latest_liquidity_stake_id(staker);
    runtime::ret(CLValue::from_t(typecast_to_string(ret)).unwrap_or_revert());
}

#[no_mangle]
fn decimals() {
    let ret = data::decimals();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn create_pair() {
    StakeableTokenStruct::default().create_pair();
}

#[no_mangle]
fn get_inflation() {
    let amount: U256 = runtime::get_named_arg("amount");
    let ret: U256 = StakeableTokenStruct::default().get_inflation(amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_amounts_out() {
    let _path: Vec<String> = runtime::get_named_arg("path");
    let mut path: Vec<Key> = Vec::new();
    for key in _path.iter() {
        path.push(Key::from_formatted_str(key).unwrap());
    }
    let amount_in: U256 = runtime::get_named_arg("amount_in");

    let amounts: Vec<U256> = StakeableTokenStruct::default().get_amounts_out(amount_in, path);
    runtime::ret(CLValue::from_t(amounts).unwrap_or_revert());
}

/// This function is to get the reserves like Reserve0, Reserve1 and Block Time Stamp
///

#[no_mangle]
fn get_reserves() {
    let (reserve0, reserve1, block_timestamp_last): (U128, U128, u64) =
        StakeableTokenStruct::default().get_reserves();
    runtime::ret(CLValue::from_t((reserve0, reserve1, block_timestamp_last)).unwrap_or_revert());
}

#[no_mangle]
/// Swap exact tokens for tokens.
///
/// Parameters-> amount_in:U256, amount_out_min:U256, path:Vec<Key>, to:Key, deadline:U256
fn swap_exact_tokens_for_tokens() {
    let deadline: U256 = runtime::get_named_arg("deadline");
    let amount_in: U256 = runtime::get_named_arg("amount_in");
    let amount_out_min: U256 = runtime::get_named_arg("amount_out_min");
    let path: Vec<String> = runtime::get_named_arg("path");
    let to: Key = runtime::get_named_arg("to");
    let amounts: Vec<U256> = StakeableTokenStruct::default().swap_exact_tokens_for_tokens(
        deadline,
        amount_in,
        amount_out_min,
        path,
        to,
    );
    runtime::ret(CLValue::from_t(typecast_to_string(amounts)).unwrap_or_revert());
}

#[no_mangle]
/// Swap exact cspr for tokens.
///
/// Parameters-> amount_out_min:U256, amount_in:U256, path:Vec<Key>, to:Key, deadline:U256, purse:URef
fn swap_exact_cspr_for_tokens() {
    let deadline: U256 = runtime::get_named_arg("deadline");
    let amount_out_min: U256 = runtime::get_named_arg("amount_out_min");
    let amount_in: U256 = runtime::get_named_arg("amount_in");
    let path: Vec<String> = runtime::get_named_arg("path");
    let to: Key = runtime::get_named_arg("to");
    let purse: URef = runtime::get_named_arg("purse");

    let amounts: Vec<U256> = StakeableTokenStruct::default().swap_exact_cspr_for_tokens(
        deadline,
        amount_out_min,
        amount_in,
        path,
        to,
        purse,
    );
    runtime::ret(CLValue::from_t(typecast_to_string(amounts)).unwrap_or_revert());
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
        "create_pair",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_inflation",
        vec![Parameter::new("amount", CLType::U256)],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_amounts_out",
        vec![
            Parameter::new("amount_in", U256::cl_type()),
            Parameter::new("path", CLType::List(Box::new(CLType::String))),
        ],
        CLType::List(Box::new(CLType::String)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_reserves",
        vec![],
        CLType::Tuple3([
            Box::new(CLType::U128),
            Box::new(CLType::U128),
            Box::new(u64::cl_type()),
        ]),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("swap_exact_tokens_for_tokens"),
        vec![
            Parameter::new("amount_in", CLType::U256),
            Parameter::new("amount_out_min", CLType::U256),
            Parameter::new("path", CLType::List(Box::new(String::cl_type()))),
            Parameter::new("to", CLType::Key),
            Parameter::new("deadline", CLType::U256),
        ],
        CLType::List(Box::new(CLType::String)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("swap_exact_cspr_for_tokens"),
        vec![
            Parameter::new("amount_out_min", CLType::U256),
            Parameter::new("amount_in", CLType::U256),
            Parameter::new("path", CLType::List(Box::new(String::cl_type()))),
            Parameter::new("to", CLType::Key),
            Parameter::new("deadline", CLType::U256),
            Parameter::new("purse", CLType::URef),
        ],
        CLType::List(Box::new(CLType::String)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "scrape_interest_Jsclient",
        vec![
            Parameter::new("stake_id", CLType::List(Box::new(CLType::String))),
            Parameter::new("scrape_days", CLType::U64),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "check_referrals_by_id_Jsclient",
        vec![
            Parameter::new("referral_id", CLType::List(Box::new(CLType::String))),
            Parameter::new("referrer", CLType::Key),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "create_stake_Jsclient",
        vec![
            Parameter::new("staked_amount", CLType::U256),
            Parameter::new("lock_days", u64::cl_type()),
            Parameter::new("referrer", CLType::Key),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "end_stake_Jsclient",
        vec![Parameter::new(
            "stake_id",
            CLType::List(Box::new(CLType::String)),
        )],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "check_mature_stake_Jsclient",
        vec![
            Parameter::new("stake_id", CLType::List(Box::new(CLType::String))),
            Parameter::new("staker", CLType::Key),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "check_stake_by_id_Jsclient",
        vec![
            Parameter::new("stake_id", CLType::List(Box::new(CLType::String))),
            Parameter::new("staker", CLType::Key),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "create_liquidity_stake_Jsclient",
        vec![Parameter::new("liquidity_tokens", CLType::U256)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "end_liquidity_stake_Jsclient",
        vec![Parameter::new(
            "liquidity_stake_id",
            CLType::List(Box::new(CLType::String)),
        )],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "check_liquidity_stake_by_id_Jsclient",
        vec![
            Parameter::new("staker", CLType::Key),
            Parameter::new("liquidity_stake_id", CLType::List(Box::new(CLType::String))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "create_stake_with_cspr_Jsclient",
        vec![
            Parameter::new("lock_days", CLType::U64),
            Parameter::new("referrer", CLType::Key),
            Parameter::new("amount", CLType::U256),
            Parameter::new("purse", CLType::URef),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "create_stake_with_token_Jsclient",
        vec![
            Parameter::new("token_address", CLType::Key),
            Parameter::new("token_amount", CLType::U256),
            Parameter::new("lock_days", CLType::U64),
            Parameter::new("referrer", CLType::Key),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "transfer_Jsclient",
        vec![
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "transfer_from_Jsclient",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "generate_id",
        vec![
            Parameter::new("x", CLType::Key),
            Parameter::new("y", CLType::U256),
            Parameter::new("z", CLType::U8),
        ],
        CLType::List(Box::new(CLType::String)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "stakes_pagination",
        vec![
            Parameter::new("staker", CLType::Key),
            Parameter::new("length", CLType::U256),
            Parameter::new("offset", CLType::U256),
        ],
        CLType::List(Box::new(CLType::List(Box::new(CLType::String)))),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "referrals_pagination",
        vec![
            Parameter::new("referrer", CLType::Key),
            Parameter::new("length", CLType::U256),
            Parameter::new("offset", CLType::U256),
        ],
        CLType::List(Box::new(CLType::List(Box::new(CLType::String)))),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "latest_referral_id",
        vec![Parameter::new("staker", CLType::Key)],
        CLType::List(Box::new(CLType::String)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "latest_stake_id",
        vec![Parameter::new("staker", CLType::Key)],
        CLType::List(Box::new(CLType::String)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "latest_liquidity_stake_id",
        vec![Parameter::new("staker", CLType::Key)],
        CLType::List(Box::new(CLType::String)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "decimals",
        vec![],
        CLType::U8,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "scrape_interest",
        vec![
            Parameter::new("stake_id", CLType::List(Box::new(CLType::String))),
            Parameter::new("scrape_days", CLType::U64),
        ],
        CLType::List(Box::new(CLType::String)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "check_referrals_by_id",
        vec![
            Parameter::new("referral_id", CLType::List(Box::new(CLType::String))),
            Parameter::new("referrer", CLType::Key),
        ],
        CLType::List(Box::new(CLType::String)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "current_stakeable_day",
        vec![],
        u64::cl_type(),
        EntryPointAccess::Public,
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
        vec![Parameter::new("update_day", u64::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_stable_usd_equivalent",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "referrer_interest",
        vec![
            Parameter::new("referral_id", CLType::List(Box::new(CLType::String))),
            Parameter::new("scrape_days", CLType::U256),
        ],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "create_stake_bulk",
        vec![
            Parameter::new("staked_amount", CLType::List(Box::new(CLType::String))),
            Parameter::new("lock_days", CLType::List(Box::new(CLType::String))),
            Parameter::new("referrer", CLType::List(Box::new(CLType::String))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "create_stake",
        vec![
            Parameter::new("staked_amount", CLType::U256),
            Parameter::new("lock_days", u64::cl_type()),
            Parameter::new("referrer", CLType::Key),
        ],
        CLType::Tuple3([
            Box::new(CLType::List(Box::new(CLType::String))),
            Box::new(CLType::U256),
            Box::new(CLType::List(Box::new(CLType::String))),
        ]),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "end_stake",
        vec![Parameter::new(
            "stake_id",
            CLType::List(Box::new(CLType::String)),
        )],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "check_mature_stake",
        vec![
            Parameter::new("stake_id", CLType::List(Box::new(CLType::String))),
            Parameter::new("staker", CLType::Key),
        ],
        CLType::Bool,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "check_mature_stake",
        vec![
            Parameter::new("stake_id", CLType::List(Box::new(CLType::String))),
            Parameter::new("staker", CLType::Key),
        ],
        CLType::Tuple3([
            Box::new(CLType::List(Box::new(CLType::String))),
            Box::new(CLType::U256),
            Box::new(CLType::Bool),
        ]),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "create_liquidity_stake",
        vec![Parameter::new("liquidity_tokens", CLType::U256)],
        CLType::List(Box::new(CLType::String)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "end_liquidity_stake",
        vec![Parameter::new(
            "liquidity_stake_id",
            CLType::List(Box::new(CLType::String)),
        )],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "check_liquidity_stake_by_id",
        vec![
            Parameter::new("staker", CLType::Key),
            Parameter::new("liquidity_stake_id", CLType::List(Box::new(CLType::String))),
        ],
        CLType::List(Box::new(CLType::String)),
        EntryPointAccess::Public,
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
        "set_stable_usd_equivalent",
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
            Box::new(CLType::List(Box::new(CLType::String))),
            Box::new(CLType::U256),
            Box::new(CLType::List(Box::new(CLType::String))),
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
            Box::new(CLType::List(Box::new(CLType::String))),
            Box::new(CLType::U256),
            Box::new(CLType::List(Box::new(CLType::String))),
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
            err: Box::new(CLType::String),
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
            err: Box::new(CLType::String),
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
            err: Box::new(CLType::String),
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
            err: Box::new(CLType::String),
        },
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points
}

// All session code must have a `call` entrypoint.
#[no_mangle]
pub extern "C" fn call() {
    // Store contract in the account's named keys. Contract name must be same for all new versions of the contracts
    let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");

    // If this is the first deployment
    if !runtime::has_key(&format!("{}_package_hash", contract_name)) {
        // Build new package.
        let (package_hash, access_token) = storage::create_contract_package_at_hash();
        // add a first version to this package
        let (contract_hash, _): (ContractHash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());

        let (domain_separator, permit_type_hash) = StakeableTokenStruct::default()
            .get_permit_type_and_domain_separator("Stakeable Token", contract_hash);

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
    // If contract package did already exist
    else {
        // get the package
        let package_hash: ContractPackageHash =
            runtime::get_key(&format!("{}_package_hash", contract_name))
                .unwrap_or_revert()
                .into_hash()
                .unwrap()
                .into();
        // create new version and install it
        let (contract_hash, _): (ContractHash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());

        // update contract hash
        runtime::put_key(
            &format!("{}_contract_hash", contract_name),
            contract_hash.into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash_wrapped", contract_name),
            storage::new_uref(contract_hash).into(),
        );
    }
}
