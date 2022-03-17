use casper_types::{ContractPackageHash, Key, URef, U128, U256};
extern crate alloc;
use crate::commons::key_names::SELF_PACKAGE_HASH;
use alloc::{
    collections::BTreeMap,
    format,
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::contract_api::storage;
use contract_utils::get_key;

pub enum StakeableEvents {
    StakeStart {
        stake_id: Vec<u32>,
        staker_address: Key,
        referral_address: Key,
        staked_amount: U256,
        stakes_shares: U256,
        referral_shares: U256,
        start_day: u64,
        lock_days: u64,
        dai_equivalent: U256,
    },
    StakeEnd {
        stake_id: Vec<u32>,
        staker_address: Key,
        referral_address: Key,
        staked_amount: U256,
        stakes_shares: U256,
        referral_shares: U256,
        reward_amount: U256,
        close_day: u64,
        penalty_amount: U256,
    },
    InterestScraped {
        stake_id: Vec<u32>,
        staker_address: Key,
        scrape_amount: U256,
        scrape_day: U256,
        stakers_penalty: U256,
        referrer_penalty: U256,
        current_stakeable_day: u64,
    },
    ReferralCollected {
        staker: Key,
        stake_id: Vec<u32>,
        referrer: Key,
        referrer_id: Vec<u32>,
        reward_amount: U256,
    },
    NewGlobals {
        total_shares: U256,
        total_staked: U256,
        share_rate: U256,
        referral_shares: U256,
        current_stakeable_day: u64,
    },
    NewSharePrice {
        new_share_price: U256,
        old_share_price: U256,
        current_stakeable_day: u64,
    },
    UniswapReserves {
        reserve_a: U128,
        reserve_b: U128,
        block_timestamp_last: u64,
    },
    LiquidityGuardStatus {
        liquidity_guard_status: bool,
    },
}

impl StakeableEvents {
    pub fn type_name(&self) -> String {
        match self {
            StakeableEvents::StakeStart {
                stake_id: _,
                staker_address: _,
                referral_address: _,
                staked_amount: _,
                stakes_shares: _,
                referral_shares: _,
                start_day: _,
                lock_days: _,
                dai_equivalent: _,
            } => "stake_start",
            StakeableEvents::StakeEnd {
                stake_id: _,
                staker_address: _,
                referral_address: _,
                staked_amount: _,
                stakes_shares: _,
                referral_shares: _,
                reward_amount: _,
                close_day: _,
                penalty_amount: _,
            } => "stake_end",
            StakeableEvents::InterestScraped {
                stake_id: _,
                staker_address: _,
                scrape_amount: _,
                scrape_day: _,
                stakers_penalty: _,
                referrer_penalty: _,
                current_stakeable_day: _,
            } => "interest_scraped",
            StakeableEvents::ReferralCollected {
                staker: _,
                stake_id: _,
                referrer: _,
                referrer_id: _,
                reward_amount: _,
            } => "referral_collected",
            StakeableEvents::NewGlobals {
                total_shares: _,
                total_staked: _,
                share_rate: _,
                referral_shares: _,
                current_stakeable_day: _,
            } => "new_globals",
            StakeableEvents::NewSharePrice {
                new_share_price: _,
                old_share_price: _,
                current_stakeable_day: _,
            } => "new_share_price",
            StakeableEvents::UniswapReserves {
                reserve_a: _,
                reserve_b: _,
                block_timestamp_last: _,
            } => "uniswap_reserves",
            StakeableEvents::LiquidityGuardStatus {
                liquidity_guard_status: _,
            } => "liquidity_guard_status",
        }
        .to_string()
    }
}

pub fn emit(stakeable_event: &StakeableEvents) {
    let mut events = Vec::new();
    let package: ContractPackageHash = get_key(SELF_PACKAGE_HASH).unwrap();
    // let package = ContractHash::from(package.into_hash().unwrap_or_default());
    match stakeable_event {
        StakeableEvents::StakeStart {
            stake_id,
            staker_address,
            referral_address,
            staked_amount,
            stakes_shares,
            referral_shares,
            start_day,
            lock_days,
            dai_equivalent,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", stakeable_event.type_name());
            event.insert("stake_id", format!("{:?}", stake_id));
            event.insert("staker_address", staker_address.to_string());
            event.insert("referral_address", referral_address.to_string());
            event.insert("staked_amount", staked_amount.to_string());
            event.insert("stakes_shares", stakes_shares.to_string());
            event.insert("referral_shares", referral_shares.to_string());
            event.insert("start_day", start_day.to_string());
            event.insert("lock_days", lock_days.to_string());
            event.insert("dai_equivalent", dai_equivalent.to_string());
            events.push(event)
        }
        StakeableEvents::StakeEnd {
            stake_id,
            staker_address,
            referral_address,
            staked_amount,
            stakes_shares,
            referral_shares,
            reward_amount,
            close_day,
            penalty_amount,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", stakeable_event.type_name());
            event.insert("stake_id", format!("{:?}", stake_id));
            event.insert("staker_address", staker_address.to_string());
            event.insert("referral_address", referral_address.to_string());
            event.insert("staked_amount", staked_amount.to_string());
            event.insert("stakes_shares", stakes_shares.to_string());
            event.insert("referral_shares", referral_shares.to_string());
            event.insert("reward_amount", reward_amount.to_string());
            event.insert("close_day", close_day.to_string());
            event.insert("penalty_amount", penalty_amount.to_string());
            events.push(event)
        }
        StakeableEvents::InterestScraped {
            stake_id,
            staker_address,
            scrape_amount,
            scrape_day,
            stakers_penalty,
            referrer_penalty,
            current_stakeable_day,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", stakeable_event.type_name());
            event.insert("stake_id", format!("{:?}", stake_id));
            event.insert("staker_address", staker_address.to_string());
            event.insert("scrape_amount", scrape_amount.to_string());
            event.insert("scrape_day", scrape_day.to_string());
            event.insert("stakers_penalty", stakers_penalty.to_string());
            event.insert("referrer_penalty", referrer_penalty.to_string());
            event.insert("current_stakeable_day", current_stakeable_day.to_string());
            events.push(event)
        }
        StakeableEvents::ReferralCollected {
            staker,
            stake_id,
            referrer,
            referrer_id,
            reward_amount,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", stakeable_event.type_name());
            event.insert("staker", staker.to_string());
            event.insert("stake_id", format!("{:?}", stake_id));
            event.insert("referrer", referrer.to_string());
            event.insert("referrer_id", format!("{:?}", referrer_id));
            event.insert("reward_amount", reward_amount.to_string());
            events.push(event);
        }
        StakeableEvents::NewGlobals {
            total_shares,
            total_staked,
            share_rate,
            referral_shares,
            current_stakeable_day,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", stakeable_event.type_name());
            event.insert("total_shares", total_shares.to_string());
            event.insert("total_staked", total_staked.to_string());
            event.insert("share_rate", share_rate.to_string());
            event.insert("referrer_shares", referral_shares.to_string());
            event.insert("current_stakeable_day", current_stakeable_day.to_string());
            events.push(event);
        }
        StakeableEvents::NewSharePrice {
            new_share_price,
            old_share_price,
            current_stakeable_day,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", stakeable_event.type_name());
            event.insert("new_share_price", new_share_price.to_string());
            event.insert("old_share_price", old_share_price.to_string());
            event.insert("current_stakeable_day", current_stakeable_day.to_string());
            events.push(event);
        }
        StakeableEvents::UniswapReserves {
            reserve_a,
            reserve_b,
            block_timestamp_last,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", stakeable_event.type_name());
            event.insert("reserve_a", reserve_a.to_string());
            event.insert("reserve_b", reserve_b.to_string());
            event.insert("block_timestamp_last", block_timestamp_last.to_string());
            events.push(event);
        }
        StakeableEvents::LiquidityGuardStatus {
            liquidity_guard_status,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", stakeable_event.type_name());
            event.insert("liquidity_guard_status", liquidity_guard_status.to_string());
            events.push(event);
        }
    };
    for event in events {
        let _: URef = storage::new_uref(event);
    }
}
