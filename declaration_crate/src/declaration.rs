extern crate alloc;
use alloc::{format, string::String, vec::Vec};

use casper_contract::contract_api::runtime;
use casper_types::{
    bytesrepr::ToBytes, contracts::ContractHash, runtime_args, ApiError, Key, RuntimeArgs, U256,
};
use contract_utils::{ContractContext, ContractStorage};

use crate::data::{self};
use globals_crate::Globals;
use hex::encode;
use renvm_sig::keccak256;
use wise_token_utils::{commons::key_names::*, declaration::parameters, error_codes};

pub trait Declaration<Storage: ContractStorage>:
    ContractContext<Storage> + Globals<Storage>
{
    // Will be called by constructor
    fn init(
        &mut self,
        launch_time: U256,
        uniswap_router: Key,
        factory: Key,
        pair_hash: Key,
        liquidity_guard: Key,
        synthetic_cspr: Key,
        wcspr: Key,
    ) {
        data::set_router_hash(uniswap_router);
        data::set_factory_hash(factory);
        data::set_pair_hash(pair_hash);
        data::set_liquidity_guard_hash(liquidity_guard);
        data::set_scspr(synthetic_cspr);
        data::set_launch_time(launch_time);
        data::set_inflation_rate(U256::from(103000)); // 3.000% (indirect -> checks through LiquidityGuard)
        data::set_liquidity_rate(U256::from(100006)); // 0.006% (indirect -> checks through LiquidityGuard)
        data::set_wcspr(wcspr);

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

    fn create_pair(&self) {
        let factory: Key = data::factory_hash();
        let pair: Key = data::pair_hash();
        let scspr: Key = data::scspr();

        let args: RuntimeArgs = runtime_args! {
            "token_a" => scspr,
            "token_b" => data::self_hash(),
            "pair_hash" => pair
        };
        let () = runtime::call_contract(
            ContractHash::from(factory.into_hash().unwrap_or_default()),
            "create_pair",
            args,
        ); // create pair just initializes the pair that is passed in.

        data::set_uniswap_pair(pair);
    }

    fn set_liquidity_stake(&self, staker: Key, id: Vec<u32>, value: Vec<u8>) {
        let liquidity_stakes = data::LiquidityStakes::instance();
        let dictionary_key = Self::_generate_key_for_dictionary(self, &staker, &id);
        liquidity_stakes.set(&dictionary_key, value);
    }

    fn get_liquidity_stake(&self, staker: Key, id: Vec<u32>) -> Vec<u8> {
        let liquidity_stakes = data::LiquidityStakes::instance();
        let dictionary_key = Self::_generate_key_for_dictionary(self, &staker, &id);
        liquidity_stakes.get(&dictionary_key)
    }

    fn launch_time(&self) -> U256 {
        data::launch_time()
    }

    fn get_stake_count(&self, staker: Key) -> U256 {
        let stake_count = data::StakeCount::instance();
        stake_count.get(&staker)
    }

    fn set_stake_count(&self, staker: Key, value: U256) {
        let stake_count = data::StakeCount::instance();
        stake_count.set(&staker, value);
    }

    fn get_referral_count(&self, referral: Key) -> U256 {
        let referral_count = data::ReferralCount::instance();
        referral_count.get(&referral)
    }

    fn set_referral_count(&self, referral: Key, value: U256) {
        let referral_count = data::ReferralCount::instance();
        referral_count.set(&referral, value);
    }

    fn get_liquidity_stake_count(&self, staker: Key) -> U256 {
        let liquidity_stake_count = data::LiquidityStakeCount::instance();
        liquidity_stake_count.get(&staker)
    }

    fn set_liquidity_stake_count(&self, staker: Key, value: U256) {
        let liquidity_stake_count = data::LiquidityStakeCount::instance();
        liquidity_stake_count.set(&staker, value);
    }

    fn get_referral_shares_to_end(&self, key: U256) -> U256 {
        let referral_shares_to_end = data::ReferralSharesToEnd::instance();
        referral_shares_to_end.get(&key)
    }

    fn set_referral_shares_to_end(&self, key: U256, value: U256) {
        let referral_shares_to_end = data::ReferralSharesToEnd::instance();
        referral_shares_to_end.set(&key, value);
    }

    fn set_scheduled_to_end(&self, key: U256, value: U256) {
        let scheduled_to_end = data::ScheduledToEnd::instance();
        scheduled_to_end.set(&key, value);
    }

    fn get_scheduled_to_end(&self, key: U256) -> U256 {
        let scheduled_to_end = data::ScheduledToEnd::instance();
        scheduled_to_end.get(&key)
    }

    fn set_total_penalties(&self, key: U256, value: U256) {
        let scheduled_to_end = data::TotalPenalties::instance();
        scheduled_to_end.set(&key, value);
    }

    fn get_total_penalties(&self, key: U256) -> U256 {
        let scheduled_to_end = data::TotalPenalties::instance();
        scheduled_to_end.get(&key)
    }

    fn set_scrapes(&self, key: String, value: U256) {
        let scrapes = data::Scrapes::instance();
        scrapes.set(&key, value);
    }

    fn get_scrapes(&self, key: String) -> U256 {
        let scrapes = data::Scrapes::instance();
        scrapes.get(&key)
    }

    fn set_inflation_rate(&self, value: U256) {
        data::set_inflation_rate(value);
    }

    fn get_inflation_rate(&self) -> U256 {
        data::inflation_rate()
    }

    fn set_liquidity_rate(&self, value: U256) {
        data::set_liquidity_rate(value);
    }

    fn get_liquidity_rate(&self) -> U256 {
        data::liquidity_rate()
    }

    fn set_liquidity_guard_status(&self, value: bool) {
        data::set_liquidity_guard_status(value);
    }

    fn get_liquidity_guard_status(&self) -> bool {
        data::liquidity_guard_status()
    }

    fn set_scspr(&self, scspr: Key) {
        data::set_scspr(scspr);
    }

    fn get_scspr(&self) -> Key {
        data::scspr()
    }

    fn get_wcspr(&self) -> Key {
        data::wcspr()
    }

    fn set_stable_usd_equivalent(&self, address: Key) {
        data::set_stable_usd_equivalent(address);
    }

    fn get_stable_usd_equivalent(&self) -> Key {
        data::stable_usd_equivalent()
    }

    fn get_uniswap_pair(&self) -> Key {
        data::uniswap_pair()
    }

    fn get_launchtime(&self) -> U256 {
        data::launch_time()
    }

    fn set_launchtime(&self, value: U256) {
        data::set_launch_time(value);
    }

    fn get_lt_balance(&self) -> U256 {
        data::lt_balance()
    }

    fn set_lt_balance(&self, value: U256) {
        data::set_lt_balance(value);
    }

    // This function is used to get the struct objects stored against key. These struct objects are returned as string.
    fn get_struct_from_key(&self, key: String, struct_name: String) -> Vec<u8> {
        if struct_name.eq(DECLARATION_STAKES_DICT) {
            let stakes = data::Stakes::instance();
            return stakes.get(&key);
        } else if struct_name.eq(DECLARATION_REFERRER_LINK_DICT) {
            let referral_link = data::ReferrerLink::instance();
            return referral_link.get(&key);
        } else if struct_name.eq(DECLARATION_CRITICAL_MASS_DICT) {
            let critical_mass = data::CriticalMass::instance();
            return critical_mass.get(&key);
        } else {
            runtime::revert(ApiError::User(
                error_codes::ErrorCodes::InvalidParameter as u16,
            ));
        }
    }

    // This function is used to set the struct objects stored against key. These struct objects are received as json string.
    fn set_struct_from_key(&self, key: String, value: Vec<u8>, struct_name: String) {
        if struct_name.eq(DECLARATION_STAKES_DICT) {
            let stakes = data::Stakes::instance();
            stakes.set(&key, value);
        } else if struct_name.eq(DECLARATION_REFERRER_LINK_DICT) {
            let referral_link = data::ReferrerLink::instance();
            referral_link.set(&key, value);
        } else if struct_name.eq(DECLARATION_CRITICAL_MASS_DICT) {
            let critical_mass = data::CriticalMass::instance();
            critical_mass.set(&key, value);
        }
    }

    // returns the struct of constants (defined in config) as a json string.
    fn get_declaration_constants(&self) -> Vec<u8> {
        let const_struct = parameters::ConstantParameters::instance();
        let struct_bytes = const_struct.clone().into_bytes().unwrap();

        struct_bytes
    }

    fn _generate_key_for_dictionary(&self, key: &Key, id: &Vec<u32>) -> String {
        let key_str = format!("{}{:?}", key, id);
        encode(keccak256(key_str.as_bytes())) // since concatinated key is too long, hash it to reduce length
    }
}
