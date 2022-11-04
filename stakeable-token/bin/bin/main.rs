#![no_main]

extern crate alloc;
use alloc::{
    boxed::Box,
    collections::BTreeSet,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, CLType, CLTyped, CLValue, EntryPoint, EntryPointAccess, EntryPointType,
    EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};
use casperlabs_contract_utils::{ContractContext, OnChainContractStorage};
use stakeable_token_crate::{functions::*, transformer_gate_keeper, *};

#[derive(Default)]
struct StakeableToken(OnChainContractStorage);
impl ContractContext<OnChainContractStorage> for StakeableToken {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}
impl STAKEABLETOKEN<OnChainContractStorage> for StakeableToken {}
impl REFERRALTOKEN<OnChainContractStorage> for StakeableToken {}
impl STAKINGTOKEN<OnChainContractStorage> for StakeableToken {}
impl LIQUIDITYTOKEN<OnChainContractStorage> for StakeableToken {}
impl DECLARATION<OnChainContractStorage> for StakeableToken {}
impl GLOBAL<OnChainContractStorage> for StakeableToken {}
impl TIMING<OnChainContractStorage> for StakeableToken {}
impl HELPER<OnChainContractStorage> for StakeableToken {}
impl SNAPSHOT<OnChainContractStorage> for StakeableToken {}
impl ERC20<OnChainContractStorage> for StakeableToken {}

impl StakeableToken {
    #[allow(clippy::too_many_arguments)]
    fn constructor(
        &mut self,
        scspr: Key,
        wcspr: Key,
        uniswap_router: Key,
        uniswap_factory: Key,
        uniswap_pair: Key,
        liquidity_guard: Key,
        contract_hash: Key,
        package_hash: Key,
    ) {
        STAKEABLETOKEN::init(
            self,
            scspr,
            wcspr,
            uniswap_router,
            uniswap_factory,
            uniswap_pair,
            liquidity_guard,
            contract_hash,
            package_hash,
        );
    }
}

#[no_mangle]
fn constructor() {
    let scspr: Key = runtime::get_named_arg("scspr");
    let wcspr: Key = runtime::get_named_arg("wcspr");
    let uniswap_router: Key = runtime::get_named_arg("uniswap_router");
    let uniswap_factory: Key = runtime::get_named_arg("uniswap_factory");
    let uniswap_pair: Key = runtime::get_named_arg("uniswap_pair");
    let liquidity_guard: Key = runtime::get_named_arg("liquidity_guard");
    let contract_hash: Key = runtime::get_named_arg("contract_hash");
    let package_hash: Key = runtime::get_named_arg("package_hash");
    StakeableToken::default().constructor(
        scspr,
        wcspr,
        uniswap_router,
        uniswap_factory,
        uniswap_pair,
        liquidity_guard,
        contract_hash,
        package_hash,
    );
}

/// @notice ability to define liquidity transformer contract
/// @dev this method renounce transformerGateKeeper access
/// @param _immutableTransformer contract address
#[no_mangle]
fn set_liquidity_transfomer() {
    let immutable_transformer: Key = runtime::get_named_arg("immutable_transformer");
    let transformer_purse: URef = runtime::get_named_arg("transformer_purse");
    StakeableToken::default().set_liquidity_transfomer(immutable_transformer, transformer_purse);
}

/// @notice allows liquidityTransformer to mint supply
/// @dev executed from liquidityTransformer upon PANCAKESWAP transfer and during reservation payout to contributors and referrers
/// @param _investorAddress address for minting stakeable tokens
/// @param _amount of tokens to mint for _investorAddress
#[no_mangle]
fn mint_supply() {
    let investor_address: Key = runtime::get_named_arg("investor_address");
    let amount: U256 = runtime::get_named_arg("amount");
    StakeableToken::default().mint_supply(investor_address, amount);
}

/// @notice allows to create stake directly with BNB if you don't have stakeable tokens method will wrap
///     your BNB to SBNB and use that amount on PANCAKESWAP returned amount of stakeable tokens will b used to stake
/// @param _lockDays amount of days it is locked for.
/// @param _referrer referrer address for +10% bonus
#[no_mangle]
fn create_stake_with_cspr() {
    let lock_days: u64 = runtime::get_named_arg("lock_days");
    let referrer: Key = runtime::get_named_arg("referrer");
    let amount: U256 = runtime::get_named_arg("amount");
    let (stake_id, start_day, referrer_id): (Vec<u32>, U256, Vec<u32>) =
        StakeableToken::default().create_stake_with_cspr(lock_days, referrer, amount);
    runtime::ret(CLValue::from_t((stake_id, start_day, referrer_id)).unwrap_or_revert());
}

#[no_mangle]
fn get_pair_address() {
    let ret: Key = StakeableToken::default().get_pair_address();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_total_staked() {
    let ret: U256 = StakeableToken::default().get_total_staked();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_liquidity_transformer() {
    let ret: Key = StakeableToken::default().get_liquidity_transformer();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_synthetic_token_address() {
    let ret: Key = StakeableToken::default().get_synthetic_token_address();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
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
    let ret = StakeableToken::default().transfer(recipient, amount);
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
    let ret = StakeableToken::default().transfer_from(owner, recipient, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
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
    StakeableToken::default().approve(spender, amount);
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
    StakeableToken::default().mint(to, amount);
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
    StakeableToken::default().burn(from, amount);
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
    let ret: U256 = StakeableToken::default().balance_of(owner);
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
    let ret: U256 = StakeableToken::default().nonce(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to return the Name of contract
///

#[no_mangle]
fn name() {
    let ret: String = StakeableToken::default().name();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to return the Symbol of contract
///

#[no_mangle]
fn symbol() {
    let ret: String = StakeableToken::default().symbol();
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
    let ret: U256 = StakeableToken::default().allowance(owner, spender);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to return the Total Supply of the contract
///

#[no_mangle]
fn total_supply() {
    let ret: U256 = StakeableToken::default().total_supply();
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

    let ret: Result<(), u32> = StakeableToken::default().increase_allowance(spender, amount);
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

    let ret: Result<(), u32> = StakeableToken::default().decrease_allowance(spender, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice package_hash of stakeable
#[no_mangle]
fn package_hash() {
    let ret: ContractPackageHash = StakeableToken::default().get_package_hash();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice current day after staking
#[no_mangle]
fn current_stakeable_day() {
    let ret: u64 = StakeableToken::default().current_stakeable_day();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice allows to activate/deactivate liquidity guard manually based on the liquidity in UNISWAP pair contract
#[no_mangle]
fn liquidity_guard_trigger() {
    StakeableToken::default().liquidity_guard_trigger();
}

/// @notice allows volunteer to offload snapshots to save on gas during next start/end stake
#[no_mangle]
fn manual_daily_snapshot() {
    StakeableToken::default().manual_daily_snapshot();
}

/// @notice allows volunteer to offload snapshots to save on gas during next start/end stake in case manualDailySnapshot reach block limit
#[no_mangle]
fn manual_daily_snapshot_point() {
    let update_day: u64 = runtime::get_named_arg("update_day");
    StakeableToken::default().manual_daily_snapshot_point(update_day);
}

#[no_mangle]
fn get_stable_usd_equivalent() {
    let ret: U256 = StakeableToken::default().get_stable_usd_equivalent();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn referrer_interest() {
    let referral_id: Vec<u32> = runtime::get_named_arg("referral_id");
    let scrape_days: U256 = runtime::get_named_arg("scrape_days");
    StakeableToken::default().referrer_interest(referral_id, scrape_days);
}

#[no_mangle]
fn referrer_interest_bulk() {
    let referral_ids: Vec<Vec<u32>> = runtime::get_named_arg("referral_ids");
    let scrape_days: Vec<U256> = runtime::get_named_arg("scrape_days");
    StakeableToken::default().referrer_interest_bulk(referral_ids, scrape_days);
}

#[no_mangle]
fn check_referrals_by_id() {
    let referrer: Key = runtime::get_named_arg("referrer");
    let referral_id: Vec<u32> = runtime::get_named_arg("referral_id");
    let (
        staker,
        stake_id,
        referrer_shares,
        referral_interest,
        is_active_referral,
        is_active_stake,
        is_mature_stake,
        is_ended_stake,
    ) = StakeableToken::default().check_referrals_by_id(referrer, referral_id);
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

/// @notice A method for a staker to create multiple stakes
/// @param _stakedAmount amount of STAKEABLE staked.
/// @param _lockDays amount of days it is locked for.
/// @param _referrer address of the referrer
#[no_mangle]
fn create_stake_bulk() {
    let staked_amount: Vec<U256> = runtime::get_named_arg("staked_amount");
    let lock_days: Vec<u64> = runtime::get_named_arg("lock_days");
    let _referrer: Vec<String> = runtime::get_named_arg("referrer");
    let mut referrer: Vec<Key> = Vec::new();
    for i in &_referrer {
        referrer.push(Key::from_formatted_str(i).unwrap());
    }
    StakeableToken::default().create_stake_bulk(staked_amount, lock_days, referrer);
}

/// @notice A method for a staker to create a stake
/// @param _stakedAmount amount of STAKEABLE staked.
/// @param _lockDays amount of days it is locked for.
/// @param _referrer address of the referrer
#[no_mangle]
fn create_stake() {
    let staked_amount: U256 = runtime::get_named_arg("staked_amount");
    let lock_days: u64 = runtime::get_named_arg("lock_days");
    let referrer: Key = runtime::get_named_arg("referrer");
    let (stake_id, start_day, referral_id): (Vec<u32>, U256, Vec<u32>) =
        StakeableToken::default().create_stake(staked_amount, lock_days, referrer);
    runtime::ret(CLValue::from_t((stake_id, start_day, referral_id)).unwrap_or_revert());
}

/// @notice A method for a staker to remove a stake belonging to his address by providing ID of a stake.
/// @param stake_id unique bytes sequence reference to the stake
#[no_mangle]
fn end_stake() {
    let stake_id: Vec<u32> = runtime::get_named_arg("stake_id");
    let ret: U256 = StakeableToken::default().end_stake(stake_id);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice alloes to scrape interest from active stake
/// @param stake_id unique bytes sequence reference to the stake
/// @param scrape_days amount of days to proccess, 0 = all
#[no_mangle]
fn scrape_interest() {
    let stake_id: Vec<u32> = runtime::get_named_arg("stake_id");
    let scrape_days: u64 = runtime::get_named_arg("scrape_days");
    let (scrape_day, scrape_amount, remaining_days, stakers_penalty, referrer_penalty) =
        StakeableToken::default().scrape_interest(stake_id, scrape_days);
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
    let stake_id: Vec<u32> = runtime::get_named_arg("stake_id");
    let ret: bool = StakeableToken::default().check_mature_stake(staker, stake_id);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn check_stake_by_id() {
    let staker: Key = runtime::get_named_arg("staker");
    let stake_id: Vec<u32> = runtime::get_named_arg("stake_id");
    let (
        start_day,
        lock_days,
        final_day,
        close_day,
        scrape_day,
        staked_amount,
        stakes_shares,
        reward_amount,
        penalty_amount,
        is_active,
        is_mature,
    ) = StakeableToken::default().check_stake_by_id(staker, stake_id);
    let ret: Vec<String> = vec![
        start_day.to_string(),
        lock_days.to_string(),
        final_day.to_string(),
        close_day.to_string(),
        scrape_day.to_string(),
        staked_amount.to_string(),
        stakes_shares.to_string(),
        reward_amount.to_string(),
        penalty_amount.to_string(),
        is_active.to_string(),
        is_mature.to_string(),
    ];
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice A method for a staker to create a liquidity stake
/// @param _liquidityTokens amount of UNI-STAKEABLE staked.
#[no_mangle]
fn create_liquidity_stake() {
    let liquidity_tokens: U256 = runtime::get_named_arg("liquidity_tokens");
    let liquidity_stake_id: Vec<u32> =
        StakeableToken::default().create_liquidity_stake(liquidity_tokens);
    runtime::ret(CLValue::from_t(liquidity_stake_id).unwrap_or_revert());
}

/// @notice A method for a staker to end a liquidity stake
/// @param _liquidityStakeID - identification number
#[no_mangle]
fn end_liquidity_stake() {
    let liquidity_stake_id: Vec<u32> = runtime::get_named_arg("liquidity_stake_id");
    let ret: U256 = StakeableToken::default().end_liquidity_stake(liquidity_stake_id);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice returns full view and details of a liquidity stake belonging to caller
/// @param _liquidityStakeID - stakeID
#[no_mangle]
fn check_liquidity_stake_by_id() {
    let staker: Key = runtime::get_named_arg("staker");
    let liquidity_stake_id: Vec<u32> = runtime::get_named_arg("liquidity_stake_id");
    let (start_day, staked_amount, reward_amount, close_day, is_active) =
        StakeableToken::default().check_liquidity_stake_by_id(staker, liquidity_stake_id);
    let ret: Vec<String> = vec![
        start_day.to_string(),
        staked_amount.to_string(),
        reward_amount.to_string(),
        close_day.to_string(),
        is_active.to_string(),
    ];
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn generate_id() {
    let x: Key = runtime::get_named_arg("x");
    let y: U256 = runtime::get_named_arg("y");
    let z: u8 = runtime::get_named_arg("z");
    let ret: Vec<u32> = StakeableToken::default().generate_id(x, y, z);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn stakes_pagination() {
    let staker: Key = runtime::get_named_arg("staker");
    let offset: U256 = runtime::get_named_arg("offset");
    let length: U256 = runtime::get_named_arg("length");
    let ret: Vec<Vec<u32>> = StakeableToken::default().stakes_pagination(staker, offset, length);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn referrals_pagination() {
    let staker: Key = runtime::get_named_arg("referrer");
    let offset: U256 = runtime::get_named_arg("offset");
    let length: U256 = runtime::get_named_arg("length");
    let ret: Vec<Vec<u32>> = StakeableToken::default().referrals_pagination(staker, offset, length);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn latest_stake_id() {
    let staker: Key = runtime::get_named_arg("staker");
    let ret = StakeableToken::default().latest_stake_id(staker);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn latest_referral_id() {
    let staker: Key = runtime::get_named_arg("staker");
    let ret: Vec<u32> = StakeableToken::default().latest_referral_id(staker);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn latest_liquidity_stake_id() {
    let staker: Key = runtime::get_named_arg("staker");
    let ret: Vec<u32> = StakeableToken::default().latest_liquidity_stake_id(staker);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn decimals() {
    let ret = StakeableToken::default().decimals();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn create_pair() {
    StakeableToken::default().create_pair();
}

#[no_mangle]
fn snapshots() {
    let key: U256 = runtime::get_named_arg("key");
    runtime::ret(CLValue::from_t(Snapshots::instance().get(&key)).unwrap_or_revert());
}

#[no_mangle]
fn rsnapshots() {
    let key: U256 = runtime::get_named_arg("key");
    runtime::ret(CLValue::from_t(Snapshots::instance().get(&key)).unwrap_or_revert());
}

#[no_mangle]
fn lsnapshots() {
    let key: U256 = runtime::get_named_arg("key");
    runtime::ret(CLValue::from_t(Snapshots::instance().get(&key)).unwrap_or_revert());
}

#[no_mangle]
fn get_scheduled_to_end() {
    let key: U256 = runtime::get_named_arg("key");
    runtime::ret(CLValue::from_t(ScheduledToEnd::instance().get(&key)).unwrap_or_revert());
}

#[no_mangle]
fn get_referral_shares_to_end() {
    let key: U256 = runtime::get_named_arg("key");
    runtime::ret(CLValue::from_t(ReferralSharesToEnd::instance().get(&key)).unwrap_or_revert());
}

#[no_mangle]
fn get_total_penalties() {
    let key: U256 = runtime::get_named_arg("key");
    runtime::ret(CLValue::from_t(TotalPenalties::instance().get(&key)).unwrap_or_revert());
}

#[no_mangle]
fn get_scrapes() {
    let key0: Key = runtime::get_named_arg("key0");
    let key1: Vec<u32> = runtime::get_named_arg("key1");
    runtime::ret(CLValue::from_t(Scrapes::instance().get(&key0, &key1)).unwrap_or_revert());
}

#[no_mangle]
fn stake_count() {
    let staker: Key = runtime::get_named_arg("staker");
    runtime::ret(CLValue::from_t(StakeCount::instance().get(&staker)).unwrap_or_revert());
}

#[no_mangle]
fn referral_count() {
    let referral: Key = runtime::get_named_arg("referral");
    runtime::ret(CLValue::from_t(ReferralCount::instance().get(&referral)).unwrap_or_revert());
}

#[no_mangle]
fn get_liquidity_rate() {
    runtime::ret(CLValue::from_t(liquidity_rate()).unwrap_or_revert());
}

#[no_mangle]
fn get_scspr() {
    runtime::ret(CLValue::from_t(scspr()).unwrap_or_revert());
}

#[no_mangle]
fn get_uniswap_pair() {
    runtime::ret(CLValue::from_t(uniswap_pair()).unwrap_or_revert());
}

#[no_mangle]
fn get_inflation_rate() {
    runtime::ret(CLValue::from_t(inflation_rate()).unwrap_or_revert());
}

#[no_mangle]
fn get_transformer_gate_keeper() {
    runtime::ret(CLValue::from_t(transformer_gate_keeper()).unwrap_or_revert());
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("scspr", CLType::Key),
            Parameter::new("wcspr", CLType::Key),
            Parameter::new("uniswap_router", CLType::Key),
            Parameter::new("uniswap_factory", CLType::Key),
            Parameter::new("uniswap_pair", CLType::Key),
            Parameter::new("liquidity_guard", CLType::Key),
            Parameter::new("contract_hash", Key::cl_type()),
            Parameter::new("package_hash", Key::cl_type()),
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
        "get_transformer_gate_keeper",
        vec![],
        Key::cl_type(),
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
        "get_liquidity_rate",
        vec![],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_scspr",
        vec![],
        CLType::Key,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_uniswap_pair",
        vec![],
        CLType::Key,
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
        "get_lt_balance",
        vec![],
        CLType::U256,
        EntryPointAccess::Public,
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
        "get_scheduled_to_end",
        vec![Parameter::new("key", U256::cl_type())],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_total_penalties",
        vec![Parameter::new("key", U256::cl_type())],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_liquidity_stake",
        vec![
            Parameter::new("staker", Key::cl_type()),
            Parameter::new("id", CLType::List(Box::new(String::cl_type()))),
        ],
        CLType::List(Box::new(u8::cl_type())),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_scrapes",
        vec![
            Parameter::new("key0", Key::cl_type()),
            Parameter::new("key1", CLType::List(Box::new(u32::cl_type()))),
        ],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_stake_count",
        vec![Parameter::new("staker", Key::cl_type())],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_referral_count",
        vec![Parameter::new("referral", Key::cl_type())],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_liquidity_stake_count",
        vec![Parameter::new("staker", Key::cl_type())],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_liquidity_guard_status",
        vec![],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_referral_shares_to_end",
        vec![Parameter::new("key", U256::cl_type())],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "snapshots",
        vec![Parameter::new("key", CLType::U256)],
        CLType::List(Box::new(SnapShot::cl_type())),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "rsnapshots",
        vec![Parameter::new("key", CLType::U256)],
        CLType::List(Box::new(SnapShot::cl_type())),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "lsnapshots",
        vec![Parameter::new("key", CLType::U256)],
        CLType::List(Box::new(SnapShot::cl_type())),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "check_stake_by_id",
        vec![
            Parameter::new("stake_id", CLType::List(Box::new(CLType::U32))),
            Parameter::new("staker", CLType::Key),
        ],
        CLType::List(Box::new(CLType::String)),
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
        CLType::List(Box::new(CLType::U32)),
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
        CLType::List(Box::new(CLType::List(Box::new(CLType::U32)))),
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
        CLType::List(Box::new(CLType::List(Box::new(CLType::U32)))),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "latest_stake_id",
        vec![Parameter::new("staker", CLType::Key)],
        CLType::List(Box::new(CLType::U32)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "latest_referral_id",
        vec![Parameter::new("staker", CLType::Key)],
        CLType::List(Box::new(CLType::U32)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "latest_liquidity_stake_id",
        vec![Parameter::new("staker", CLType::Key)],
        CLType::List(Box::new(CLType::U32)),
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
            Parameter::new("stake_id", CLType::List(Box::new(CLType::U32))),
            Parameter::new("scrape_days", CLType::U64),
        ],
        CLType::List(Box::new(CLType::String)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "check_referrals_by_id",
        vec![
            Parameter::new("referral_id", CLType::List(Box::new(CLType::U32))),
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
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "referrer_interest",
        vec![
            Parameter::new("referral_id", CLType::List(Box::new(CLType::U32))),
            Parameter::new("scrape_days", CLType::U256),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "referrer_interest_bulk",
        vec![
            Parameter::new(
                "referral_id",
                CLType::List(Box::new(CLType::List(Box::new(CLType::U32)))),
            ),
            Parameter::new("scrape_days", CLType::List(Box::new(CLType::U256))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "create_stake_bulk",
        vec![
            Parameter::new("staked_amount", CLType::List(Box::new(CLType::U256))),
            Parameter::new("lock_days", CLType::List(Box::new(CLType::U64))),
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
            Box::new(CLType::List(Box::new(CLType::U32))),
            Box::new(CLType::U256),
            Box::new(CLType::List(Box::new(CLType::U32))),
        ]),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "end_stake",
        vec![Parameter::new(
            "stake_id",
            CLType::List(Box::new(CLType::U32)),
        )],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "check_mature_stake",
        vec![
            Parameter::new("stake_id", CLType::List(Box::new(CLType::U32))),
            Parameter::new("staker", CLType::Key),
        ],
        CLType::Bool,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "create_liquidity_stake",
        vec![Parameter::new("liquidity_tokens", CLType::U256)],
        CLType::List(Box::new(CLType::U32)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "end_liquidity_stake",
        vec![Parameter::new(
            "liquidity_stake_id",
            CLType::List(Box::new(CLType::U32)),
        )],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "check_liquidity_stake_by_id",
        vec![
            Parameter::new("staker", CLType::Key),
            Parameter::new("liquidity_stake_id", CLType::List(Box::new(CLType::U32))),
        ],
        CLType::List(Box::new(CLType::String)),
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

        let scspr: Key = runtime::get_named_arg("scspr");
        let wcspr: Key = runtime::get_named_arg("wcspr");
        let uniswap_router: Key = runtime::get_named_arg("uniswap_router");
        let uniswap_factory: Key = runtime::get_named_arg("uniswap_factory");
        let uniswap_pair: Key = runtime::get_named_arg("uniswap_pair");
        let liquidity_guard: Key = runtime::get_named_arg("liquidity_guard");

        // Prepare constructor args
        let constructor_args = runtime_args! {
            "scspr" => scspr,
            "wcspr" => wcspr,
            "uniswap_router" => uniswap_router,
            "uniswap_factory" => uniswap_factory,
            "uniswap_pair" => uniswap_pair,
            "liquidity_guard" => liquidity_guard,
            "contract_hash" => Key::from(contract_hash),
            "package_hash" => Key::from(package_hash),
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
