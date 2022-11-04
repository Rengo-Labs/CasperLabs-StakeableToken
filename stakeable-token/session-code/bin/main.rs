#![no_std]
#![no_main]

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;
use alloc::{string::String, vec::Vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::ToBytes, runtime_args, ApiError, CLTyped, Key, RuntimeArgs, URef, U256,
};
use common::keys::*;

// Key is the same a destination
fn store<T: CLTyped + ToBytes>(key: &str, value: T) {
    // Store `value` under a new unforgeable reference.
    let value_ref: URef = storage::new_uref(value);

    // Wrap the unforgeable reference in a value of type `Key`.
    let value_key: Key = value_ref.into();

    // Store this key under the name "special_value" in context-local storage.
    runtime::put_key(key, value_key);
}

#[no_mangle]
pub extern "C" fn call() {
    let entrypoint: String = runtime::get_named_arg("entrypoint");
    let package_hash: Key = runtime::get_named_arg("package_hash");

    match entrypoint.as_str() {
        // Stakeable_Token
        CREATE_STAKE_WITH_CSPR => {
            let lock_days: u64 = runtime::get_named_arg("lock_days");
            let referrer: Key = runtime::get_named_arg("referrer");
            let amount: U256 = runtime::get_named_arg("amount");
            let ret: (Vec<u32>, U256, Vec<u32>) = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                CREATE_STAKE_WITH_CSPR,
                runtime_args! {
                    "lock_days" => lock_days,
                    "referrer" => referrer,
                    "amount" => amount
                },
            );
            store(CREATE_STAKE_WITH_CSPR, ret);
        }
        CREATE_STAKE_WITH_TOKEN => {
            let token_address: Key = runtime::get_named_arg("token_address");
            let token_amount: U256 = runtime::get_named_arg("token_amount");
            let lock_days: u64 = runtime::get_named_arg("lock_days");
            let referrer: Key = runtime::get_named_arg("referrer");
            let ret: (Vec<u32>, U256, Vec<u32>) = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                CREATE_STAKE_WITH_TOKEN,
                runtime_args! {
                    "token_address" => token_address,
                    "token_amount" => token_amount,
                    "lock_days" => lock_days,
                    "referrer" => referrer
                },
            );
            store(CREATE_STAKE_WITH_TOKEN, ret);
        }
        TRANSFER => {
            let recipient: Key = runtime::get_named_arg("recipient");
            let amount: U256 = runtime::get_named_arg("amount");
            let ret: Result<(), u32> = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                TRANSFER,
                runtime_args! {
                    "recipient" => recipient,
                    "amount" => amount
                },
            );
            store(TRANSFER, ret);
        }
        TRANSFER_FROM => {
            let owner: Key = runtime::get_named_arg("owner");
            let recipient: Key = runtime::get_named_arg("recipient");
            let amount: U256 = runtime::get_named_arg("amount");
            let ret: Result<(), u32> = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                TRANSFER_FROM,
                runtime_args! {
                    "owner" => owner,
                    "recipient" => recipient,
                    "amount" => amount
                },
            );
            store(TRANSFER_FROM, ret);
        }
        CHECK_REFERRALS_BY_ID => {
            let referrer: Key = runtime::get_named_arg("referrer");
            let referral_id: Vec<String> = runtime::get_named_arg("referral_id");
            let ret: Vec<String> = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                CHECK_REFERRALS_BY_ID,
                runtime_args! {
                    "referral_id" => referral_id,
                    "referrer" => referrer
                },
            );
            store(CHECK_REFERRALS_BY_ID, ret);
        }
        CREATE_STAKE => {
            let staked_amount: U256 = runtime::get_named_arg("staked_amount");
            let lock_days: u64 = runtime::get_named_arg("lock_days");
            let referrer: Key = runtime::get_named_arg("referrer");
            let ret: (Vec<u32>, U256, Vec<u32>) = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                CREATE_STAKE,
                runtime_args! {
                    "staked_amount" => staked_amount,
                    "lock_days" => lock_days,
                    "referrer" => referrer
                },
            );
            store(CREATE_STAKE, ret);
        }
        END_STAKE => {
            let stake_id: Vec<String> = runtime::get_named_arg("stake_id");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                END_STAKE,
                runtime_args! {
                    "stake_id" => stake_id
                },
            );
            store(END_STAKE, ret);
        }

        SCRAPE_INTEREST => {
            let stake_id: Vec<String> = runtime::get_named_arg("stake_id");
            let scrape_days: u64 = runtime::get_named_arg("scrape_days");
            let ret: Vec<String> = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                SCRAPE_INTEREST,
                runtime_args! {
                    "stake_id" => stake_id,
                    "scrape_days" => scrape_days
                },
            );
            store(SCRAPE_INTEREST, ret);
        }
        CHECK_MATURE_STAKE => {
            let staker: Key = runtime::get_named_arg("staker");
            let stake_id: Vec<String> = runtime::get_named_arg("stake_id");
            let ret: bool = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                CHECK_MATURE_STAKE,
                runtime_args! {
                    "staker" => staker,
                    "stake_id" => stake_id
                },
            );
            store(CHECK_MATURE_STAKE, ret);
        }
        CHECK_STAKE_BY_ID => {
            let staker: Key = runtime::get_named_arg("staker");
            let stake_id: Vec<String> = runtime::get_named_arg("stake_id");
            let ret: (Vec<u8>, U256, bool) = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                CHECK_STAKE_BY_ID,
                runtime_args! {
                    "staker" => staker,
                    "stake_id" => stake_id
                },
            );
            store(CHECK_STAKE_BY_ID, ret);
        }
        CREATE_LIQUIDITY_STAKE => {
            let liquidity_tokens: U256 = runtime::get_named_arg("liquidity_tokens");
            let ret: Vec<u32> = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                CREATE_LIQUIDITY_STAKE,
                runtime_args! {
                    "liquidity_tokens" => liquidity_tokens
                },
            );
            store(CREATE_LIQUIDITY_STAKE, ret);
        }
        END_LIQUIDITY_STAKE => {
            let liquidity_stake_id: Vec<String> = runtime::get_named_arg("liquidity_stake_id");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                END_LIQUIDITY_STAKE,
                runtime_args! {
                    "liquidity_stake_id" => liquidity_stake_id
                },
            );
            store(END_LIQUIDITY_STAKE, ret);
        }
        CHECK_LIQUIDITY_STAKE_BY_ID => {
            let staker: Key = runtime::get_named_arg("staker");
            let liquidity_stake_id: Vec<String> = runtime::get_named_arg("liquidity_stake_id");
            let ret: Vec<u8> = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                CHECK_LIQUIDITY_STAKE_BY_ID,
                runtime_args! {
                    "staker" => staker,
                    "liquidity_stake_id" => liquidity_stake_id
                },
            );
            store(CHECK_LIQUIDITY_STAKE_BY_ID, ret);
        }
        GET_STABLE_USD_EQUIVALENT => {
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                GET_STABLE_USD_EQUIVALENT,
                runtime_args! {},
            );
            store(GET_STABLE_USD_EQUIVALENT, ret);
        }
        _ => runtime::revert(ApiError::UnexpectedKeyVariant),
    };
}
