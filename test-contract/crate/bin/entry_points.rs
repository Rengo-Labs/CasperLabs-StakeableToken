#![no_main]
#![no_std]

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
    runtime_args, CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints,
    Group, Key, Parameter, RuntimeArgs, URef, U256,
};
use contract_utils::{set_key, ContractContext, OnChainContractStorage};

pub mod mappings;

// use busd_equivalent::BUSDEquivalent;
use declaration_crate::Declaration;
use globals_crate::Globals;
use helper_crate::Helper;
// use liquidity_guard::LiquidityGuard;
use bep20_crate::BEP20;
use liquidity_token_crate::LiquidityToken;
use referral_token_crate::ReferralToken;
use snapshot_crate::Snapshot;
use staking_token_crate::StakingToken;
use timing_crate::Timing;

// use transfer_helper::TransferHelper;
use wise_token_utils::commons::key_names::*;

#[derive(Default)]
struct Test(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for Test {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

// impl BUSDEquivalent<OnChainContractStorage> for Test {}
impl BEP20<OnChainContractStorage> for Test {}
impl Declaration<OnChainContractStorage> for Test {}
impl Globals<OnChainContractStorage> for Test {}
impl Helper<OnChainContractStorage> for Test {}
// impl LiquidityGuard<OnChainContractStorage> for Test {}
impl LiquidityToken<OnChainContractStorage> for Test {}
impl ReferralToken<OnChainContractStorage> for Test {}
impl Snapshot<OnChainContractStorage> for Test {}
impl StakingToken<OnChainContractStorage> for Test {}
impl Timing<OnChainContractStorage> for Test {}
// impl TransferHelper<OnChainContractStorage> for Test {}

impl Test {
    fn constructor(
        &mut self,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
        launch_time: U256,
        uniswap_router: Key,
        factory: Key,
        pair_hash: Key,
        liquidity_guard: Key,
        synthetic_cspr: Key,
        wcspr: Key,
        bep20: Key,
    ) {
        BEP20::init(self, "MyBep20".to_string(), "bep20".to_string());
        set_key(SELF_PACKAGE_HASH, package_hash);
        set_key(SELF_CONTRACT_HASH, contract_hash);
        Declaration::init(
            self,
            launch_time,
            uniswap_router,
            factory,
            pair_hash,
            liquidity_guard,
            synthetic_cspr,
            wcspr,
        );
        Globals::init(self);
        Helper::init(self);
        LiquidityToken::init(self, synthetic_cspr, pair_hash, liquidity_guard);
        ReferralToken::init(self);
        Snapshot::init(self, synthetic_cspr, pair_hash, liquidity_guard);
        StakingToken::init(self);
        Timing::init(self);
    }
}

#[no_mangle]
// Constructor is used for internal inititializations. Calling it from outside is not allowed.
fn constructor() {
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let launch_time: U256 = runtime::get_named_arg("launch_time");
    let uniswap_router: Key = runtime::get_named_arg("uniswap_router");
    let factory: Key = runtime::get_named_arg("factory");
    let pair_hash: Key = runtime::get_named_arg("pair_hash");
    let liquidity_guard: Key = runtime::get_named_arg("liquidity_guard");
    let synthetic_cspr: Key = runtime::get_named_arg("synthetic_cspr");
    let wcspr: Key = runtime::get_named_arg("wcspr");
    let bep20: Key = runtime::get_named_arg("bep20");

    Test::default().constructor(
        contract_hash,
        package_hash,
        launch_time,
        uniswap_router,
        factory,
        pair_hash,
        liquidity_guard,
        synthetic_cspr,
        wcspr,
        bep20,
    );
}

/////////////////
/// /// BEP20 // //
/// /// /// ///
///
///  
#[no_mangle]
fn bep20_mint() {
    let amount: U256 = runtime::get_named_arg("amount");
    let account: Key = runtime::get_named_arg("account");

    BEP20::_mint(&Test::default(), account, amount);
}
/////////////////
// DECLARATION //
/////////////////
#[no_mangle]
fn set_liquidity_stake() {
    let staker: Key = runtime::get_named_arg("staker");
    let id: Vec<u32> = runtime::get_named_arg("id");
    let value: Vec<u8> = runtime::get_named_arg("value");
    Declaration::set_liquidity_stake(&Test::default(), staker, id, value);
}

#[no_mangle]
fn get_liquidity_stake() {
    let staker: Key = runtime::get_named_arg("staker");
    let id: Vec<u32> = runtime::get_named_arg("id");

    let ret: Vec<u8> = Declaration::get_liquidity_stake(&Test::default(), staker, id);
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn launch_time() {
    let ret: U256 = Declaration::launch_time(&Test::default());
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn get_stake_count() {
    let staker: Key = runtime::get_named_arg("staker");
    let ret: U256 = Declaration::get_stake_count(&Test::default(), staker);
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn set_stake_count() {
    let staker: Key = runtime::get_named_arg("staker");
    let value: U256 = runtime::get_named_arg("value");
    Declaration::set_stake_count(&Test::default(), staker, value);
}

#[no_mangle]
fn get_referral_count() {
    let referrer: Key = runtime::get_named_arg("referrer");
    let ret: U256 = Declaration::get_referral_count(&Test::default(), referrer);
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn set_referral_count() {
    let referrer: Key = runtime::get_named_arg("referrer");
    let value: U256 = runtime::get_named_arg("value");

    Declaration::set_referral_count(&Test::default(), referrer, value);
}

#[no_mangle]
fn get_liquidity_stake_count() {
    let staker: Key = runtime::get_named_arg("staker");
    let ret: U256 = Declaration::get_liquidity_stake_count(&Test::default(), staker);
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn set_liquidity_stake_count() {
    let staker: Key = runtime::get_named_arg("staker");
    let value: U256 = runtime::get_named_arg("value");
    Declaration::set_liquidity_stake_count(&Test::default(), staker, value);
}

#[no_mangle]
fn get_struct_from_key() {
    let key: String = runtime::get_named_arg("key");
    let struct_name: String = runtime::get_named_arg("struct_name");
    let ret: Vec<u8> = Declaration::get_struct_from_key(&Test::default(), key, struct_name);
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn set_struct_from_key() {
    let key: String = runtime::get_named_arg("key");
    let value: Vec<u8> = runtime::get_named_arg("value");
    let struct_name: String = runtime::get_named_arg("struct_name");
    Declaration::set_struct_from_key(&Test::default(), key, value, struct_name);
}

#[no_mangle]
fn set_referral_shares_to_end() {
    let key: U256 = runtime::get_named_arg("key");
    let value: U256 = runtime::get_named_arg("value");
    Declaration::set_referral_shares_to_end(&Test::default(), key, value);
}

#[no_mangle]
fn get_referral_shares_to_end() {
    let key: U256 = runtime::get_named_arg("key");
    let ret: U256 = Declaration::get_referral_shares_to_end(&Test::default(), key);
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn set_scheduled_to_end() {
    let key: U256 = runtime::get_named_arg("key");
    let value: U256 = runtime::get_named_arg("value");
    Declaration::set_scheduled_to_end(&Test::default(), key, value);
}

#[no_mangle]
fn get_scheduled_to_end() {
    let key: U256 = runtime::get_named_arg("key");
    let ret: U256 = Declaration::get_scheduled_to_end(&Test::default(), key);
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn set_total_penalties() {
    let key: U256 = runtime::get_named_arg("key");
    let value: U256 = runtime::get_named_arg("value");
    Declaration::set_total_penalties(&Test::default(), key, value);
}

#[no_mangle]
fn get_total_penalties() {
    let key: U256 = runtime::get_named_arg("key");
    let ret: U256 = Declaration::get_total_penalties(&Test::default(), key);
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn get_declaration_constants() {
    let ret: Vec<u8> = Declaration::get_declaration_constants(&Test::default());
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn set_inflation_rate() {
    let value: U256 = runtime::get_named_arg("value");
    Declaration::set_inflation_rate(&Test::default(), value);
}

#[no_mangle]
fn get_inflation_rate() {
    let ret: U256 = Declaration::get_inflation_rate(&Test::default());
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn set_liquidity_rate() {
    let value: U256 = runtime::get_named_arg("value");
    Declaration::set_liquidity_rate(&Test::default(), value);
}

#[no_mangle]
fn get_liquidity_rate() {
    let ret: U256 = Declaration::get_liquidity_rate(&Test::default());
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn set_liquidity_guard_status() {
    let value: bool = runtime::get_named_arg("value");
    Declaration::set_liquidity_guard_status(&Test::default(), value);
}

#[no_mangle]
fn get_liquidity_guard_status() {
    let ret: bool = Declaration::get_liquidity_guard_status(&Test::default());
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn set_scspr() {
    let value: Key = runtime::get_named_arg("scspr");
    Declaration::set_scspr(&Test::default(), value);
}

#[no_mangle]
fn get_scspr() {
    let ret: Key = Declaration::get_scspr(&Test::default());
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn get_wcspr() {
    let ret: Key = Declaration::get_wcspr(&Test::default());
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn set_busd_eq() {
    let value: Key = runtime::get_named_arg("busd_eq");
    Declaration::set_busd_eq(&Test::default(), value);
}

#[no_mangle]
fn get_busd_eq() {
    let ret: Key = Declaration::get_busd_eq(&Test::default());
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn create_pair() {
    Declaration::create_pair(&mut Test::default());
}

#[no_mangle]
fn get_unsiwap_pair() {
    let ret: Key = Declaration::get_uniswap_pair(&Test::default());
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn set_lt_balance() {
    let value: U256 = runtime::get_named_arg("value");
    Declaration::set_lt_balance(&Test::default(), value);
}

#[no_mangle]
fn get_lt_balance() {
    let ret: U256 = Declaration::get_lt_balance(&Test::default());
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn get_launchtime() {
    let ret: U256 = Declaration::get_launchtime(&Test::default());
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn set_launchtime() {
    let value: U256 = runtime::get_named_arg("value");
    Declaration::set_launchtime(&Test::default(), value);
}

#[no_mangle]
fn set_scrapes() {
    let key: String = runtime::get_named_arg("key");
    let value: U256 = runtime::get_named_arg("value");
    Declaration::set_scrapes(&Test::default(), key, value);
}

#[no_mangle]
fn get_scrapes() {
    let key: String = runtime::get_named_arg("key");
    let ret: U256 = Declaration::get_scrapes(&Test::default(), key);
    mappings::set_key(&mappings::result(), ret);
}

/////////////
// GLOBALS //
/////////////
#[no_mangle]
fn increase_globals() {
    let _staked: U256 = runtime::get_named_arg("_staked");
    let _shares: U256 = runtime::get_named_arg("_shares");
    let _rshares: U256 = runtime::get_named_arg("_rshares");
    Globals::increase_globals(&mut Test::default(), _staked, _shares, _rshares);
}

#[no_mangle]
fn decrease_globals() {
    let _staked: U256 = runtime::get_named_arg("_staked");
    let _shares: U256 = runtime::get_named_arg("_shares");
    let _rshares: U256 = runtime::get_named_arg("_rshares");
    Globals::decrease_globals(&mut Test::default(), _staked, _shares, _rshares);
}

#[no_mangle]
fn set_globals() {
    let field: String = runtime::get_named_arg("field");
    let value: U256 = runtime::get_named_arg("value");
    Globals::set_globals(&mut Test::default(), field, value);
}

#[no_mangle]
fn get_globals() {
    let field: String = runtime::get_named_arg("field");
    let ret: U256 = Globals::get_globals(&mut Test::default(), field);
    mappings::set_key(&mappings::result(), ret);
}

////////////
// HELPER //
////////////
///
///
#[no_mangle]
fn generate_stake_id() {
    let staker: Key = runtime::get_named_arg("staker");
    let ret = Helper::generate_stake_id(&Test::default(), staker);
    mappings::set_key(&mappings::result(), ret);
}
#[no_mangle]
fn generate_referral_id() {
    let referrer: Key = runtime::get_named_arg("referrer");
    let ret = Helper::generate_referral_id(&Test::default(), referrer);
    mappings::set_key(&mappings::result(), ret);
}
#[no_mangle]
fn get_lock_days() {
    let stake: Vec<u8> = runtime::get_named_arg("stake");
    let ret = Helper::get_lock_days(&Test::default(), stake);
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn generate_liquidity_stake_id() {
    let staker: Key = runtime::get_named_arg("staker");
    let ret: Vec<u32> = Helper::generate_liquidity_stake_id(&Test::default(), staker);
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn increase_liquidity_stake_count() {
    let staker: Key = runtime::get_named_arg("staker");
    Helper::increase_liquidity_stake_count(&Test::default(), staker);
}

#[no_mangle]
fn stake_not_started() {
    let stake_bytes: Vec<u8> = runtime::get_named_arg("stake");
    let ret = Helper::stake_not_started(&Test::default(), stake_bytes);
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn transfer_from() {
    let token: Key = runtime::get_named_arg("token");
    let recipient: Key = runtime::get_named_arg("recipient");
    let owner: Key = runtime::get_named_arg("owner");
    let amount: U256 = runtime::get_named_arg("amount");
    let ret: Result<(), u32> =
        Helper::transfer_from(&Test::default(), token, owner, recipient, amount);
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn stake_ended() {
    let stake: Vec<u8> = runtime::get_named_arg("stake");
    let ret: bool = Helper::stake_ended(&Test::default(), stake);
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn days_diff() {
    let start_date: U256 = runtime::get_named_arg("start_date");
    let end_date: U256 = runtime::get_named_arg("end_date");
    let ret: U256 = Helper::days_diff(&Test::default(), start_date, end_date);
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn is_mature_stake() {
    let stake: Vec<u8> = runtime::get_named_arg("stake");
    let ret: bool = Helper::is_mature_stake(&Test::default(), stake);
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn days_left() {
    let stake: Vec<u8> = runtime::get_named_arg("stake");
    let ret: U256 = Helper::days_left(&Test::default(), stake);
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn not_critical_mass_referrer() {
    let referrer: Key = runtime::get_named_arg("referrer");
    let ret: bool = Helper::not_critical_mass_referrer(&Test::default(), referrer);
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn calculation_day() {
    let stake: Vec<u8> = runtime::get_named_arg("stake");
    let ret: U256 = Helper::calculation_day(&Test::default(), stake);
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn not_past() {
    let day: U256 = runtime::get_named_arg("day");
    let ret: bool = Helper::not_past(&Test::default(), day);
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn not_future() {
    let day: U256 = runtime::get_named_arg("day");
    let ret: bool = Helper::not_future(&Test::default(), day);
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn stakes_pagination() {
    let staker: Key = runtime::get_named_arg("staker");
    let offset: U256 = runtime::get_named_arg("offset");
    let length: U256 = runtime::get_named_arg("length");
    let ret: Vec<Vec<u32>> = Helper::stakes_pagination(&Test::default(), staker, offset, length);
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn referrals_pagination() {
    let referrer: Key = runtime::get_named_arg("referrer");
    let offset: U256 = runtime::get_named_arg("offset");
    let length: U256 = runtime::get_named_arg("length");
    let ret: Vec<Vec<u32>> =
        Helper::referrals_pagination(&Test::default(), referrer, offset, length);
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn starting_day() {
    let stake: Vec<u8> = runtime::get_named_arg("stake");
    let ret: U256 = Helper::starting_day(&Test::default(), stake);
    mappings::set_key(&mappings::result(), ret);
}

/////////////////////
// LIQUIDITY_TOKEN //
/////////////////////
#[no_mangle]
fn end_liquidity_stake() {
    let liquidity_stake_id: Vec<u32> = runtime::get_named_arg("id");
    let ret: U256 = LiquidityToken::_end_liquidity_stake(&Test::default(), liquidity_stake_id);
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn check_liquidity_stake_by_id() {
    let staker: Key = runtime::get_named_arg("staker");
    let liquidity_stake_id: Vec<u32> = runtime::get_named_arg("id");
    let ret: Vec<u8> =
        LiquidityToken::_check_liquidity_stake_by_id(&Test::default(), staker, liquidity_stake_id);
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn create_liquidity_stake() {
    let liquidity_tokens: U256 = runtime::get_named_arg("liquidity_tokens");
    let ret: Vec<u32> = LiquidityToken::_create_liquidity_stake(&Test::default(), liquidity_tokens);
    mappings::set_key(&mappings::result(), ret);
}

////////////////////
// REFERRAL_TOKEN //
////////////////////
#[no_mangle]
fn add_referrer_shares_to_end() {
    let final_day: U256 = runtime::get_named_arg("final_day");
    let shares: U256 = runtime::get_named_arg("shares");
    ReferralToken::_add_referrer_shares_to_end(&mut Test::default(), final_day, shares);
}
#[no_mangle]
fn remove_referrer_shares_to_end() {
    let final_day: U256 = runtime::get_named_arg("final_day");
    let shares: U256 = runtime::get_named_arg("shares");
    ReferralToken::_remove_referrer_shares_to_end(&mut Test::default(), final_day, shares);
}
#[no_mangle]
fn add_critical_mass() {
    let referrer: Key = runtime::get_named_arg("referrer");
    let dai_equivalent: U256 = runtime::get_named_arg("dai_equivalent");
    ReferralToken::_add_critical_mass(&mut Test::default(), referrer, dai_equivalent);
}

#[no_mangle]
fn remove_critical_mass() {
    let referrer: Key = runtime::get_named_arg("referrer");
    let dai_equivalent: U256 = runtime::get_named_arg("dai_equivalent");
    let start_day: U256 = runtime::get_named_arg("start_day");
    ReferralToken::_remove_critical_mass(&mut Test::default(), referrer, dai_equivalent, start_day);
}

#[no_mangle]
fn referral_token_get_busd_equivalent() {
    let ret: U256 = ReferralToken::get_busd_equivalent(&mut Test::default());
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn referrer_interest() {
    let referral_id: Vec<u32> = runtime::get_named_arg("referral_id");
    let scrape_days: U256 = runtime::get_named_arg("scrape_days");
    ReferralToken::referrer_interest(&mut Test::default(), referral_id, scrape_days);
}

#[no_mangle]
fn referrer_interest_bulk() {
    let referral_ids: Vec<Vec<u32>> = runtime::get_named_arg("referral_ids");
    let scrape_days: Vec<U256> = runtime::get_named_arg("scrape_days");
    ReferralToken::referrer_interest_bulk(&mut Test::default(), referral_ids, scrape_days);
}

#[no_mangle]
fn _referrer_interest() {
    let referrer: Key = runtime::get_named_arg("referrer");
    let referral_id: Vec<u32> = runtime::get_named_arg("referral_id");
    let scrape_days: U256 = runtime::get_named_arg("scrape_days");
    ReferralToken::_referrer_interest(&mut Test::default(), referrer, referral_id, scrape_days);
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
    ): (Key, Vec<u32>, U256, U256, bool, bool, bool, bool) =
        ReferralToken::check_referrals_by_id(&mut Test::default(), referrer, referral_id);
    // mappings::set_key(
    //     &mappings::result(),
    //     (
    //         staker,
    //         stake_id,
    //         referrer_shares,
    //         referral_interest,
    //         is_active_referral,
    //         is_active_stake,
    //         is_mature_stake,
    //         is_ended_stake,
    //     ),
    // );
    mappings::set_key("staker", staker);
    mappings::set_key("stake_id", stake_id);
    mappings::set_key("is_active_stake", is_active_stake);
    mappings::set_key("is_mature_stake", is_mature_stake);
}

//////////////
// SNAPSHOT //
//////////////
#[no_mangle]
fn manual_daily_snapshot_point() {
    let update_day: u64 = runtime::get_named_arg("update_day");
    Snapshot::manual_daily_snapshot_point(&mut Test::default(), update_day);
}

#[no_mangle]
fn liquidity_guard_trigger() {
    Snapshot::liquidity_guard_trigger(&mut Test::default());
}

#[no_mangle]
fn manual_daily_snapshot() {
    Snapshot::manual_daily_snapshot(&mut Test::default());
}

#[no_mangle]
fn snapshot_get_struct_from_key() {
    let key: U256 = runtime::get_named_arg("key");
    let struct_name: String = runtime::get_named_arg("struct_name");
    let ret: Vec<u8> = Snapshot::get_struct_from_key(&mut Test::default(), &key, struct_name);
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn snapshot_trigger() {
    Snapshot::_snapshot_trigger(&mut Test::default());
}

#[no_mangle]
fn snapshot_set_struct_from_key() {
    let key: U256 = runtime::get_named_arg("key");
    let struct_name: String = runtime::get_named_arg("struct_name");
    let value: Vec<u8> = runtime::get_named_arg("value");
    Snapshot::set_struct_from_key(&mut Test::default(), &key, value, struct_name);
}

///////////////////
// STAKING_TOKEN //
///////////////////
#[no_mangle]
fn create_stake_bulk() {
    let staked_amount: Vec<U256> = runtime::get_named_arg("staked_amount");
    let lock_days: Vec<u64> = runtime::get_named_arg("lock_days");
    let referrer: Vec<Key> = runtime::get_named_arg("referrer");
    StakingToken::create_stake_bulk(&mut Test::default(), staked_amount, lock_days, referrer);
}
#[no_mangle]
fn create_stake() {
    let staked_amount: U256 = runtime::get_named_arg("staked_amount");
    let lock_days: u64 = runtime::get_named_arg("lock_days");
    let referrer: Key = runtime::get_named_arg("referrer");
    let (stake_id, start_day, referral_id) =
        StakingToken::create_stake(&mut Test::default(), staked_amount, lock_days, referrer);
    mappings::set_key(&mappings::result(), (stake_id, start_day, referral_id));
}

#[no_mangle]
fn end_stake() {
    let stake_id: Vec<u32> = runtime::get_named_arg("stake_id");
    let ret: U256 = StakingToken::end_stake(&mut Test::default(), stake_id);
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn check_mature_stake() {
    let staker: Key = runtime::get_named_arg("staker");
    let stake_id: Vec<u32> = runtime::get_named_arg("stake_id");
    let ret: bool = StakingToken::check_mature_stake(&mut Test::default(), staker, stake_id);
    mappings::set_key(&mappings::result(), ret);
}   

#[no_mangle]    
fn check_stake_by_id() {
    let staker = runtime::get_named_arg("staker");
    let stake_id: Vec<u32> = runtime::get_named_arg("stake_id");
    let (stake, penalty_amount, is_mature): (Vec<u8>, U256, bool) =
        StakingToken::check_stake_by_id(&mut Test::default(), staker, stake_id);
    mappings::set_key(&mappings::result(), (stake, penalty_amount, is_mature));
}

////////////
// TIMING //
////////////
#[no_mangle]
fn current_wise_day() {
    let ret: u64 = Timing::current_wise_day(&mut Test::default());
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn _current_wise_day() {
    let ret: u64 = Timing::current_wise_day_only(&mut Test::default());
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn _previous_wise_day() {
    let ret: u64 = Timing::previous_wise_day(&mut Test::default());
    mappings::set_key(&mappings::result(), ret);
}

#[no_mangle]
fn _next_wise_day() {
    let ret: u64 = Timing::next_wise_day(&mut Test::default());
    mappings::set_key(&mappings::result(), ret);
}

///////////
// BEP20 //
///////////

#[no_mangle]
fn approve() {
    let token: Key = runtime::get_named_arg("token");
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    let ret: bool = runtime::call_contract(
        token.into_hash().unwrap_or_revert().into(),
        "approve",
        runtime_args! {
            "spender" => spender,
            "amount" => amount
        },
    );
    mappings::set_key(&mappings::result(), ret);
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    // BEP20
    entry_points.add_entry_point(EntryPoint::new(
        "bep20_mint",
        vec![
            Parameter::new("account", CLType::Key),
            Parameter::new("amount", CLType::U256),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    /////////////////
    // DECLARATION //
    /////////////////
    entry_points.add_entry_point(EntryPoint::new(
        "launch_time",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_stake_count",
        vec![Parameter::new("staker", CLType::Key)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_stake_count",
        vec![
            Parameter::new("staker", CLType::Key),
            Parameter::new("value", CLType::U256),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_referral_count",
        vec![Parameter::new("referrer", CLType::Key)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_referral_count",
        vec![
            Parameter::new("referrer", CLType::Key),
            Parameter::new("value", CLType::U256),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_liquidity_stake_count",
        vec![Parameter::new("staker", CLType::Key)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_liquidity_stake_count",
        vec![
            Parameter::new("staker", CLType::Key),
            Parameter::new("value", CLType::U256),
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
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_struct_from_key",
        vec![
            Parameter::new("key", CLType::String),
            Parameter::new("value", CLType::List(Box::new(u8::cl_type()))),
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
            Parameter::new("value", CLType::U256),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_referral_shares_to_end",
        vec![Parameter::new("key", CLType::U256)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_scheduled_to_end",
        vec![
            Parameter::new("key", CLType::U256),
            Parameter::new("value", CLType::U256),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_scheduled_to_end",
        vec![Parameter::new("key", CLType::U256)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_total_penalties",
        vec![
            Parameter::new("key", CLType::U256),
            Parameter::new("value", CLType::U256),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_total_penalties",
        vec![Parameter::new("key", CLType::U256)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_inflation_rate",
        vec![Parameter::new("value", CLType::U256)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_inflation_rate",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_liquidity_rate",
        vec![Parameter::new("value", CLType::U256)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_liquidity_rate",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_liquidity_guard_status",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_liquidity_guard_status",
        vec![Parameter::new("value", CLType::Bool)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_declaration_constants",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_scspr",
        vec![Parameter::new("scspr", CLType::Key)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_scspr",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_wcspr",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_busd_eq",
        vec![Parameter::new("busd_eq", CLType::Key)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_busd_eq",
        vec![],
        <()>::cl_type(),
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
        "get_unsiwap_pair",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_launchtime",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_launchtime",
        vec![Parameter::new("value", CLType::U256)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_lt_balance",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_lt_balance",
        vec![Parameter::new("value", CLType::U256)],
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
        vec![Parameter::new("key", CLType::String)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_liquidity_stake",
        vec![
            Parameter::new("staker", Key::cl_type()),
            Parameter::new("id", CLType::List(Box::new(u32::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_liquidity_stake",
        vec![
            Parameter::new("staker", Key::cl_type()),
            Parameter::new("id", CLType::List(Box::new(u32::cl_type()))),
            Parameter::new("value", CLType::List(Box::new(u8::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    /////////////
    // GLOBALS //
    /////////////
    entry_points.add_entry_point(EntryPoint::new(
        "increase_globals",
        vec![
            Parameter::new("_staked", U256::cl_type()),
            Parameter::new("_shares", U256::cl_type()),
            Parameter::new("_rshares", U256::cl_type()),
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
            Parameter::new("_rshares", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_globals",
        vec![
            Parameter::new("field", String::cl_type()),
            Parameter::new("value", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_globals",
        vec![Parameter::new("field", String::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    ////////////
    // HELPER //
    ////////////
    entry_points.add_entry_point(EntryPoint::new(
        "generate_stake_id",
        vec![Parameter::new("staker", CLType::Key)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "generate_referral_id",
        vec![Parameter::new("referrer", CLType::Key)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "stake_ended",
        vec![Parameter::new(
            "stake",
            CLType::List(Box::new(u8::cl_type())),
        )],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "days_diff",
        vec![
            Parameter::new("start_date", CLType::U256),
            Parameter::new("end_date", CLType::U256),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "days_left",
        vec![Parameter::new(
            "stake",
            CLType::List(Box::new(u8::cl_type())),
        )],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "is_mature_stake",
        vec![Parameter::new(
            "stake",
            CLType::List(Box::new(u8::cl_type())),
        )],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "not_critical_mass_referrer",
        vec![Parameter::new("referrer", CLType::Key)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "calculation_day",
        vec![Parameter::new(
            "stake",
            CLType::List(Box::new(u8::cl_type())),
        )],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "not_past",
        vec![Parameter::new("day", CLType::U256)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "not_future",
        vec![Parameter::new("day", CLType::U256)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "starting_day",
        vec![Parameter::new(
            "stake",
            CLType::List(Box::new(u8::cl_type())),
        )],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "stakes_pagination",
        vec![
            Parameter::new("staker", Key::cl_type()),
            Parameter::new("offset", CLType::U256),
            Parameter::new("length", CLType::U256),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "referrals_pagination",
        vec![
            Parameter::new("referrer", Key::cl_type()),
            Parameter::new("offset", CLType::U256),
            Parameter::new("length", CLType::U256),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer_from",
        vec![
            Parameter::new("token", Key::cl_type()),
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "generate_liquidity_stake_id",
        vec![Parameter::new("staker", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "increase_liquidity_stake_count",
        vec![Parameter::new("staker", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "stake_not_started",
        vec![Parameter::new(
            "stake",
            CLType::List(Box::new(u8::cl_type())),
        )],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_lock_days",
        vec![Parameter::new(
            "stake",
            CLType::List(Box::new(u8::cl_type())),
        )],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    /////////////////////
    // LIQUIDITY_TOKEN //
    /////////////////////
    entry_points.add_entry_point(EntryPoint::new(
        "create_liquidity_stake",
        vec![Parameter::new("liquidity_tokens", U256::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "end_liquidity_stake",
        vec![Parameter::new("id", CLType::List(Box::new(u32::cl_type())))],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "check_liquidity_stake_by_id",
        vec![Parameter::new("staker", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    ////////////////////
    // REFERRAL_TOKEN //
    ////////////////////
    entry_points.add_entry_point(EntryPoint::new(
        "add_referrer_shares_to_end",
        vec![
            Parameter::new("final_day", U256::cl_type()),
            Parameter::new("shares", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "remove_referrer_shares_to_end",
        vec![
            Parameter::new("final_day", U256::cl_type()),
            Parameter::new("shares", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "add_critical_mass",
        vec![
            Parameter::new("referrer", Key::cl_type()),
            Parameter::new("dai_equivalent", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "remove_critical_mass",
        vec![
            Parameter::new("referrer", Key::cl_type()),
            Parameter::new("dai_equivalent", U256::cl_type()),
            Parameter::new("start_day", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "referral_token_get_busd_equivalent",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "referrer_interest",
        vec![
            Parameter::new("referral_id", CLType::List(Box::new(u32::cl_type()))),
            Parameter::new("scrape_days", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "referrer_interest_bulk",
        vec![
            Parameter::new(
                "referral_ids",
                CLType::List(Box::new(CLType::List(Box::new(u32::cl_type())))),
            ),
            Parameter::new("scrape_days", CLType::List(Box::new(u32::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "_referrer_interest",
        vec![
            Parameter::new("referrer", Key::cl_type()),
            Parameter::new("referral_id", CLType::List(Box::new(u32::cl_type()))),
            Parameter::new("scrape_days", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "check_referrals_by_id",
        vec![
            Parameter::new("referrer", Key::cl_type()),
            Parameter::new("referral_id", CLType::List(Box::new(u32::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    //////////////
    // SNAPSHOT //
    //////////////
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
        vec![Parameter::new("update_day", CLType::U64)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "snapshot_get_struct_from_key",
        vec![
            Parameter::new("key", CLType::U256),
            Parameter::new("struct_name", CLType::String),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "snapshot_set_struct_from_key",
        vec![
            Parameter::new("key", CLType::U256),
            Parameter::new("struct_name", CLType::String),
            Parameter::new("value", CLType::List(Box::new(u8::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "snapshot_trigger",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    ///////////////////
    // STAKING_TOKEN //
    ///////////////////
    entry_points.add_entry_point(EntryPoint::new(
        "create_stake",
        vec![
            Parameter::new("staked_amount", U256::cl_type()),
            Parameter::new("lock_days", u64::cl_type()),
            Parameter::new("referrer", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "end_stake",
        vec![Parameter::new(
            "stake_id",
            CLType::List(Box::new(u32::cl_type())),
        )],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "create_stake_bulk",
        vec![
            Parameter::new("staked_amount", Key::cl_type()),
            Parameter::new("lock_days", Key::cl_type()),
            Parameter::new("referrer", ContractHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "check_mature_stake",
        vec![
            Parameter::new("staker", CLType::Key),
            Parameter::new("stake_id", CLType::List(Box::new(u32::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "check_stake_by_id",
        vec![
            Parameter::new("staker", CLType::Key),
            Parameter::new("stake_id", CLType::List(Box::new(u32::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    ////////////
    // TIMING //
    ////////////
    entry_points.add_entry_point(EntryPoint::new(
        "current_wise_day",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "_current_wise_day",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "_previous_wise_day",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "_next_wise_day",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    ///////////
    // BEP20 //
    ///////////
    entry_points.add_entry_point(EntryPoint::new(
        "approve",
        vec![
            Parameter::new("token", Key::cl_type()),
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}

// All session code must have a `call` entrypoint.
#[no_mangle]
pub extern "C" fn call() {
    let launch_time: U256 = runtime::get_named_arg("launch_time");
    let uniswap_router: Key = runtime::get_named_arg("uniswap_router");
    let factory: Key = runtime::get_named_arg("factory");
    let pair_hash: Key = runtime::get_named_arg("pair_hash");
    let liquidity_guard: Key = runtime::get_named_arg("liquidity_guard");
    let synthetic_cspr: Key = runtime::get_named_arg("synthetic_cspr");
    let wcspr: Key = runtime::get_named_arg("wcspr");
    let bep20: Key = runtime::get_named_arg("bep20");

    let (package_hash, access_token) = storage::create_contract_package_at_hash();
    let (contract_hash, _): (ContractHash, _) =
        storage::add_contract_version(package_hash, get_entry_points(), Default::default());

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "contract_hash" => contract_hash,
        "package_hash" => package_hash,
        "launch_time" => launch_time,
        "uniswap_router" => uniswap_router,
        "factory" => factory,
        "pair_hash" => pair_hash,
        "liquidity_guard" => liquidity_guard,
        "synthetic_cspr" => synthetic_cspr,
        "wcspr" => wcspr,
        "bep20" => bep20
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
