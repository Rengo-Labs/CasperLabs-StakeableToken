extern crate alloc;
use alloc::{string::String};

use casper_contract::{ contract_api::{runtime}};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},Key,  U256, runtime_args, RuntimeArgs};
use contract_utils::{ContractContext, ContractStorage};

use crate::data::{self};
use crate::config::parameters::*;

pub trait Declaration<Storage: ContractStorage>: ContractContext<Storage> 
{
    // Will be called by constructor
    fn init(&mut self, contract_hash: Key, package_hash: ContractPackageHash, uniswap_router: Key, factory: Key, pair_hash: Key, liquidity_guard: Key, synthetic_bnb: Key, launch_time: U256)
    {
        data::set_package_hash(package_hash);
        data::set_self_hash(contract_hash);
        data::set_router_hash(uniswap_router);
        data::set_factory_hash(factory);
        data::set_pair_hash(pair_hash);
        data::set_liquidity_guard_hash(liquidity_guard);
        data::set_synthetic_bnb_hash(synthetic_bnb);
        data::set_launch_time(launch_time);

        data::StakeCount::init();
        data::ReferralCount::init();
        data::LiquidityStakeCount::init();
        data::ScheduledToEnd::init();
        data::ReferralSharesToEnd::init();
        data::TotalPenalties::init();
        data::CriticalMass::init();
        data::Scrapes::init();
        data::Stakes::init();
        data::ReferrerLink::init();
        data::LiquidityStakes::init();
    }

    fn create_pair() 
    {
        let factory: Key = data::factory_hash();
        let pair: Key = data::pair_hash();
        let synthetic_bnb: Key = data::synthetic_bnb_hash();

        let args: RuntimeArgs = runtime_args!
        {
            "token_a" => synthetic_bnb,
            "token_b" => data::self_hash(),
            "pair_hash" => pair
        };
        let () = runtime::call_contract(ContractHash::from(factory.into_hash().unwrap_or_default()), "create_pair", args);
    }

    fn launch_time(&self) -> U256
    {
        data::launch_time()
    }

    fn get_stake_count(&self, staker: Key) -> U256
    {
        let stake_count = data::StakeCount::instance();
        stake_count.get(&staker)
    }

    fn set_stake_count(&self, staker: Key, value: U256)
    {
        let stake_count = data::StakeCount::instance();
        stake_count.set(&staker, value);
    }

    fn get_referral_count(&self, referral: Key) -> U256
    {
        let referral_count = data::ReferralCount::instance();
        referral_count.get(&referral)
    }

    fn set_referral_count(&self, referral: Key, value: U256)
    {
        let referral_count = data::StakeCount::instance();
        referral_count.set(&referral, value);
    }

    fn get_liquidity_stake_count(&self, staker: Key) -> U256
    {
        let liquidity_stake_count = data::LiquidityStakeCount::instance();
        liquidity_stake_count.get(&staker)
    }

    fn set_liquidity_stake_count(&self, staker: Key, value: U256)
    {
        let liquidity_stake_count = data::LiquidityStakeCount::instance();
        liquidity_stake_count.set(&staker, value);
    }

    fn get_struct_from_key(&self, key: String, struct_name: String) -> String
    {
        if struct_name.eq(data::STAKES) {
            let stakes = data::Stakes::instance();
            return stakes.get(&key);
        }
        else if struct_name.eq(data::REFERRER_LINK) {
            let referral_link = data::ReferrerLink::instance();
            return referral_link.get(&key);
        }
        else{
            String::from("")
        }
    }
}