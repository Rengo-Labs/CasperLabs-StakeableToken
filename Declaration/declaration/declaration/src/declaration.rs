extern crate alloc;
use alloc::{string::String};

use casper_contract::{ contract_api::{runtime}};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},Key,  U256, runtime_args, RuntimeArgs};
use contract_utils::{ContractContext, ContractStorage};

use crate::data::{self};
use crate::config::parameters;

pub trait Declaration<Storage: ContractStorage>: ContractContext<Storage> 
{
    // Will be called by constructor
    fn init(&mut self, contract_hash: Key, package_hash: ContractPackageHash, launch_time: U256, uniswap_router: Key, factory: Key, pair_hash: Key, liquidity_guard: Key, synthetic_bnb: Key, WBNB: Key)
    {
        data::set_package_hash(package_hash);
        data::set_self_hash(contract_hash);
        data::set_router_hash(uniswap_router);
        data::set_factory_hash(factory);
        data::set_pair_hash(pair_hash);
        data::set_liquidity_guard_hash(liquidity_guard);
        data::set_synthetic_bnb_hash(synthetic_bnb);
        data::set_launch_time(launch_time);
        data::set_inflation_rate(U256::from(103000));           // 3.000% (indirect -> checks through LiquidityGuard)
        data::set_liquidity_rate(U256::from(100006));           // 0.006% (indirect -> checks through LiquidityGuard)
        data::set_wbnb(WBNB);

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
        let () = runtime::call_contract(ContractHash::from(factory.into_hash().unwrap_or_default()), "create_pair", args);          // create pair just initializes the pair that is passed in.

        data::set_pancake_pair(pair);
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

    fn get_referral_shares_to_end(&self, key: U256) -> U256
    {
        let referral_shares_to_end = data::ReferralSharesToEnd::instance();
        referral_shares_to_end.get(&key)
    }

    fn set_referral_shares_to_end(&self, key: U256, value: U256)
    {
        let referral_shares_to_end = data::ReferralSharesToEnd::instance();
        referral_shares_to_end.set(&key, value);
    }

    fn set_scheduled_to_end(&self, key: U256, value: U256)
    {
        let scheduled_to_end = data::ScheduledToEnd::instance();
        scheduled_to_end.set(&key, value);
    }

    fn get_scheduled_to_end(&self, key: U256) -> U256
    {
        let scheduled_to_end = data::ScheduledToEnd::instance();
        scheduled_to_end.get(&key)
    }

    fn set_total_penalties(&self, key: U256, value: U256)
    {
        let scheduled_to_end = data::TotalPenalties::instance();
        scheduled_to_end.set(&key, value);
    }

    fn get_total_penalties(&self, key: U256) -> U256
    {
        let scheduled_to_end = data::TotalPenalties::instance();
        scheduled_to_end.get(&key)
    }

    fn set_scrapes(&self, key: String, value: U256)
    {
        let scrapes = data::Scrapes::instance();
        scrapes.set(&key, value);
    }

    fn get_scrapes(&self, key: String) -> U256
    {
        let scrapes = data::Scrapes::instance();
        scrapes.get(&key)
    }

    fn set_inflation_rate(&self, value: U256)
    {
        data::set_inflation_rate(value);
    }

    fn get_inflation_rate(&self) -> U256
    {
        data::inflation_rate()
    }

    fn set_liquidity_rate(&self, value: U256)
    {
        data::set_liquidity_rate(value);
    }

    fn get_liquidity_rate(&self) -> U256
    {
        data::liquidity_rate()
    }

    fn set_liquidity_guard_status(&self, value: bool)
    {
        data::set_liquidity_guard_status(value);
    }

    fn get_liquidity_guard_status(&self) -> bool
    {
        data::liquidity_guard_status()
    }
    
    fn set_sbnb(&self, sbnb: Key)
    {
        data::set_sbnb(sbnb);
    }

    fn get_sbnb(&self) -> Key
    {
        data::sbnb()
    }

    fn get_wbnb(&self) -> Key
    {
        data::wbnb()
    }

    fn set_busd_eq(&self, address: Key)
    {
        data::set_busd_eq(address);
    }

    fn get_busd_eq(&self) -> Key
    {
        data::busd_eq()
    }
    
    fn get_pancake_pair(&self) -> Key
    {
        data::pancake_pair()
    }

    fn get_launchtime(&self) -> U256
    {
        data::launch_time()
    }

    fn set_launchtime(&self, value: U256)
    {
        data::set_launch_time(value);
    }

    fn get_lt_balance(&self) -> U256
    {
        data::lt_balance()
    }

    fn set_lt_balance(&self, value: U256)
    {
        data::set_lt_balance(value);
    }

    
    // This function is used to get the struct objects stored against key. These struct objects are returned as string.
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
        else if struct_name.eq(data::CRITICAL_MASS) {
            let critical_mass = data::CriticalMass::instance();
            return critical_mass.get(&key);
        }
        else {
            String::from("")
        }
    }

    // This function is used to set the struct objects stored against key. These struct objects are received as json string.
    fn set_struct_from_key(&self, key: String, value: String, struct_name: String)
    {
        if struct_name.eq(data::STAKES) {
            let stakes = data::Stakes::instance();
            stakes.set(&key, value);
        }
        else if struct_name.eq(data::REFERRER_LINK) {
            let referral_link = data::ReferrerLink::instance();
            referral_link.set(&key, value);
        }
        else if struct_name.eq(data::CRITICAL_MASS) {
            let critical_mass = data::CriticalMass::instance();
            critical_mass.set(&key, value);
        }
    }

    // returns the struct of constants (defined in config) as a json string.
    fn get_declaration_constants(&self)->String
    {
        let const_struct = parameters::ConstantParameters::instance();
        let json_string = serde_json::to_string(&const_struct).unwrap();

        json_string
    }
}