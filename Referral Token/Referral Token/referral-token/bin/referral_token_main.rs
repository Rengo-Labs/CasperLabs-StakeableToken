#![no_main]
#![no_std]

extern crate alloc;

use alloc::{boxed::Box, collections::BTreeSet, format, vec, vec::Vec};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLType, CLTyped, CLValue, ContractHash, EntryPoint, EntryPointAccess,
    EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};
use contract_utils::{ContractContext, OnChainContractStorage};
use referral_token::{self, REFERRALTOKEN};

#[derive(Default)]
struct ReferralToken(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for ReferralToken {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl REFERRALTOKEN<OnChainContractStorage> for ReferralToken {}

impl ReferralToken {
    fn constructor(
        &mut self,
        declaration_hash: Key,
        timing_hash: Key,
        helper_hash: Key,
        bep20_hash: Key,
        snapshot_hash: Key,
        contract_hash: ContractHash,
    ) {
        REFERRALTOKEN::init(
            self,
            declaration_hash,
            timing_hash,
            helper_hash,
            bep20_hash,
            snapshot_hash,
            Key::from(contract_hash),
        );
    }
}

#[no_mangle]
fn constructor() {
    let declaration_hash: Key = runtime::get_named_arg("declaration_hash");
    let timing_hash: Key = runtime::get_named_arg("timing_hash");
    let helper_hash: Key = runtime::get_named_arg("helper_hash");
    let bep20_hash: Key = runtime::get_named_arg("bep20_hash");
    let snapshot_hash: Key = runtime::get_named_arg("snapshot_hash");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    ReferralToken::default().constructor(
        declaration_hash,
        timing_hash,
        helper_hash,
        bep20_hash,
        snapshot_hash,
        contract_hash,
    );
}

#[no_mangle]
fn _add_referrer_shares_to_end() {
    let _final_day: U256 = runtime::get_named_arg("_final_day");
    let _shares: U256 = runtime::get_named_arg("_shares");
    ReferralToken::default()._add_referrer_shares_to_end(_final_day, _shares);
}
#[no_mangle]
fn _remove_referrer_shares_to_end() {
    let _final_day: U256 = runtime::get_named_arg("_final_day");
    let _shares: U256 = runtime::get_named_arg("_shares");
    ReferralToken::default()._remove_referrer_shares_to_end(_final_day, _shares);
}
#[no_mangle]
fn _add_critical_mass() {
    let _referrer: Key = runtime::get_named_arg("_referrer");
    let _dai_equivalent: U256 = runtime::get_named_arg("_dai_equivalent");
    ReferralToken::default()._add_critical_mass(_referrer, _dai_equivalent);
}

#[no_mangle]
fn _remove_critical_mass() {
    let _referrer: Key = runtime::get_named_arg("_referrer");
    let _dai_equivalent: U256 = runtime::get_named_arg("_dai_equivalent");
    let _start_day: U256 = runtime::get_named_arg("_start_day");
    ReferralToken::default()._remove_critical_mass(_referrer, _dai_equivalent, _start_day);
}
#[no_mangle]
fn get_busd_equivalent() {
    let ret: U256 = ReferralToken::default().get_busd_equivalent();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn referrer_interest() {
    let _referral_id: u32 = runtime::get_named_arg("_referral_id");
    let _scrape_days: U256 = runtime::get_named_arg("_scrape_days");
    ReferralToken::default().referrer_interest(_referral_id, _scrape_days);
}
#[no_mangle]
fn referrer_interest_bulk() {
    let _referral_ids: Vec<u32> = runtime::get_named_arg("_referral_ids");
    let _scrape_days: Vec<U256> = runtime::get_named_arg("_scrape_days");
    ReferralToken::default().referrer_interest_bulk(_referral_ids, _scrape_days);
}
#[no_mangle]
fn _referrer_interest() {
    let _referrer: Key = runtime::get_named_arg("_referrer");
    let _referral_id: u32 = runtime::get_named_arg("_referral_id");
    let _scrape_days: U256 = runtime::get_named_arg("_scrape_days");
    ReferralToken::default()._referrer_interest(_referrer, _referral_id, _scrape_days);
}

#[no_mangle]
fn check_referrals_by_id() {
    let _referrer: Key = runtime::get_named_arg("_referrer");
    let _referral_id: u32 = runtime::get_named_arg("_referral_id");
    let (
        staker,
        stake_id,
        referrer_shares,
        referral_interest,
        is_active_referral,
        is_active_stake,
        is_mature_stake,
        is_ended_stake,
    ): (Key, u32, U256, U256, bool, bool, bool, bool) =
        ReferralToken::default().check_referrals_by_id(_referrer, _referral_id);
    // runtime::ret(
    //     CLValue::from_t((
    //         staker,
    //         stake_id,
    //         referrer_shares,
    //         referral_interest,
    //         is_active_referral,
    //         is_active_stake,
    //         is_mature_stake,
    //         is_ended_stake,
    //     ))
    //     .unwrap_or_revert(),
    // );
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("declaration_hash", Key::cl_type()),
            Parameter::new("timing_hash", Key::cl_type()),
            Parameter::new("helper_hash", Key::cl_type()),
            Parameter::new("bep20_hash", Key::cl_type()),
            Parameter::new("snapshot_hash", Key::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "_add_referrer_shares_to_end",
        vec![
            Parameter::new("_final_day", U256::cl_type()),
            Parameter::new("_shares", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "_remove_referrer_shares_to_end",
        vec![
            Parameter::new("_final_day", U256::cl_type()),
            Parameter::new("_shares", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "_add_critical_mass",
        vec![
            Parameter::new("_referrer", Key::cl_type()),
            Parameter::new("_dai_equivalent", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "_remove_critical_mass",
        vec![
            Parameter::new("_referrer", Key::cl_type()),
            Parameter::new("_dai_equivalent", U256::cl_type()),
            Parameter::new("_satrt_day", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_busd_equivalent",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "referrer_interest",
        vec![
            Parameter::new("_referral_id", u32::cl_type()),
            Parameter::new("_scrape_days", U256::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "referrer_interest_bulk",
        vec![
            Parameter::new("_referral_ids", CLType::Option(Box::new(u32::cl_type()))),
            Parameter::new("_scrape_days", CLType::Option(Box::new(U256::cl_type()))),
        ],
        U256::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "_referrer_interest",
        vec![
            Parameter::new("_referrer", Key::cl_type()),
            Parameter::new("_referral_id", u32::cl_type()),
            Parameter::new("_scrape_days", U256::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "check_referrals_by_id",
        vec![
            Parameter::new("_referrer", Key::cl_type()),
            Parameter::new("_referral_id", u32::cl_type()),
        ],
        CLType::Tuple1([
            Box::new(CLType::Key),
            // Box::new(u32::cl_type()),
            // Box::new(CLType::U256),
            // Box::new(CLType::U256),
            // Box::new(bool::cl_type()),
            // Box::new(bool::cl_type()),
            // Box::new(bool::cl_type()),
            // Box::new(bool::cl_type()),
        ]),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points
}

#[no_mangle]
fn call() {
    // Build new package with initial a first version of the contract.
    let (package_hash, access_token) = storage::create_contract_package_at_hash();
    let (contract_hash, _) =
        storage::add_contract_version(package_hash, get_entry_points(), Default::default());
    let declaration_hash: Key = runtime::get_named_arg("declaration_hash");
    let timing_hash: Key = runtime::get_named_arg("timing_hash");
    let helper_hash: Key = runtime::get_named_arg("helper_hash");
    let bep20_hash: Key = runtime::get_named_arg("bep20_hash");

    let snapshot_hash: Key = runtime::get_named_arg("snapshot_hash");

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "declaration_hash"=>declaration_hash,
        "timing_hash"=>timing_hash,
        "helper_hash"=>helper_hash,
        "bep20_hash"=>bep20_hash,
        "snapshot_hash"=>snapshot_hash,
        "contract_hash" => contract_hash,
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
